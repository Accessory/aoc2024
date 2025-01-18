use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::AddAssign;
use std::path::Path;
use std::rc::Rc;
use utils::grid::Grid;
use utils::grid_point::GridPoint;
use utils::{get_input_path, parse_into_char_vector_vector};

#[derive(Debug)]
#[allow(dead_code)]
struct Region {
    name: char,
    area: usize,
    fences: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Region2 {
    name: char,
    area: Vec<GridPoint>,
    fences: usize,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

fn run(input_file: &Path) {
    // Preamble

    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    let mut regions = Vec::new();
    let mut region_grid: Grid<Option<Rc<RefCell<Region>>>> =
        Grid::with_width_height(max_x, max_y, None);

    // Prepare
    for y in 0..max_y {
        for x in 0..max_x {
            if region_grid.data[y][x].is_some() {
                continue;
            }

            let name = grid.data[y][x];

            let mut queue = Vec::new();
            queue.push(GridPoint { x, y });

            let current_region = Rc::new(RefCell::new(Region {
                name,
                area: 0,
                fences: 0,
            }));

            regions.push(current_region.clone());
            let mut current_region_cell = current_region.borrow_mut();

            while let Some(position) = queue.pop() {
                if region_grid.data[position.y][position.x].is_some() {
                    continue;
                }

                let (fences, neighbors) =
                    get_fences_and_neighbors(position, name, &grid, max_x, max_y);

                current_region_cell.area.add_assign(1);
                current_region_cell.fences.add_assign(fences);

                neighbors.into_iter().for_each(|i| queue.push(i));

                region_grid.set_from_point(&position, Some(current_region.clone()));
            }
        }
    }

    // Solve
    let mut result = 0;
    for region in regions {
        result += region.borrow().area * region.borrow().fences;
    }
    // Result
    println!("Result of part 1 is {}", result);
}

#[derive(Debug, Default)]
struct RegionCounter {
    visited_points: HashSet<GridPoint>,
    border_count: usize,
    vertex_count: usize,
}

fn run2(input_file: &Path) {
    // Preamble
    let mut visited: HashSet<GridPoint> = HashSet::new();
    // let mut total_borders = 0;
    let mut total_vertices = 0;

    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    for y in 0..max_y {
        for x in 0..max_x {
            let position = GridPoint { x, y };
            if !visited.contains(&position) {
                let region_counter = traverse_region(
                    position,
                    &mut visited,
                    Rc::new(RefCell::new(RegionCounter::default())),
                    &grid,
                    max_x,
                    max_y,
                );
                // total_borders += region_counter.borrow().visited_points.len()
                //     * region_counter.borrow().border_count;
                total_vertices += region_counter.borrow().visited_points.len()
                    * region_counter.borrow().vertex_count;
            }
        }
    }

    println!("Result of part 2 is {}", total_vertices);
}

fn traverse_region(
    position: GridPoint,
    visited: &mut HashSet<GridPoint>,
    counter: Rc<RefCell<RegionCounter>>,
    grid: &Grid<char>,
    max_x: usize,
    max_y: usize,
) -> Rc<RefCell<RegionCounter>> {
    if !visited.insert(position) {
        return counter;
    }
    counter.borrow_mut().visited_points.insert(position);

    let value = grid.data[position.y][position.x];
    let neighbours = position.generate_non_diagonal_neighbors_with_check(max_x, max_y);
    let reachable: Vec<GridPoint> = neighbours
        .iter()
        .filter(|n| grid.data[n.y][n.x] == value).copied()
        .collect();
    let borders = 4 - reachable.len();

    let vertices: usize = calculate_vertex_count(position, value, grid, borders);
    counter.borrow_mut().border_count += borders;
    counter.borrow_mut().vertex_count += vertices;

    for neighbour in reachable {
        traverse_region(neighbour, visited, counter.clone(), grid, max_x, max_y);
    }

    counter
}

fn calculate_vertex_count(
    position: GridPoint,
    value: char,
    grid: &Grid<char>,
    border_count: usize,
) -> usize {
    let up = *grid
        .get(position.x, position.y.checked_sub(1).unwrap_or(usize::MAX))
        .unwrap_or(&'#');
    let down = *grid.get(position.x, position.y + 1).unwrap_or(&'#');
    let left = *grid
        .get(position.x.checked_sub(1).unwrap_or(usize::MAX), position.y)
        .unwrap_or(&'#');
    let right = *grid.get(position.x + 1, position.y).unwrap_or(&'#');

    let up_left = *grid
        .get(
            position.x.checked_sub(1).unwrap_or(usize::MAX),
            position.y.checked_sub(1).unwrap_or(usize::MAX),
        )
        .unwrap_or(&'#');
    let up_right = *grid
        .get(
            position.x + 1,
            position.y.checked_sub(1).unwrap_or(usize::MAX),
        )
        .unwrap_or(&'#');
    let down_left = *grid
        .get(
            position.x.checked_sub(1).unwrap_or(usize::MAX),
            position.y + 1,
        )
        .unwrap_or(&'#');
    let down_right = *grid.get(position.x + 1, position.y + 1).unwrap_or(&'#');

    let concave = [
        up_left != value && up == value && left == value,
        up_right != value && up == value && right == value,
        down_right != value && down == value && right == value,
        down_left != value && down == value && left == value,
    ]
    .iter()
    .filter(|&b| *b)
    .count();

    if border_count == 4 {
        return 4 + concave;
    }

    if border_count == 3 {
        return 2 + concave;
    }

    let convex = if border_count == 2
        && !((up == down && up == value) || (left == right && left == value))
    {
        1
    } else {
        0
    };

    concave + convex
}

fn main() {
    let input_file = get_input_path(env!("CARGO_MANIFEST_DIR"));

    println!("Running {}", env!("CARGO_PKG_NAME"));
    println!("InputFile: {}", input_file.display());

    run(input_file.as_path());
    run2(input_file.as_path());
}

fn get_fences_and_neighbors(
    position: GridPoint,
    name: char,
    grid: &Grid<char>,
    max_x: usize,
    max_y: usize,
) -> (usize, Vec<GridPoint>) {
    let mut neighbors = Vec::with_capacity(4);
    for neighbor in position.generate_non_diagonal_neighbors_with_check(max_x, max_y) {
        if name == grid.data[neighbor.y][neighbor.x] {
            neighbors.push(neighbor)
        }
    }
    (4 - neighbors.len(), neighbors)
}

#[cfg(test)]
mod main_test {
    use utils::{get_test_input_2_path, get_test_input_path};

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        run(&get_test_input_path(env!("CARGO_MANIFEST_DIR")));
    }

    #[test]
    fn test_input_part_2() {
        run2(&get_test_input_2_path(env!("CARGO_MANIFEST_DIR")));
    }
}
