use std::cell::RefCell;
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

impl Region2 {
    fn has(&self, x: usize, y: usize) -> bool {
        self.area.iter().any(|ap| ap.x == x && ap.y == y)
    }
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

fn run2(input_file: &Path) {
    // Preamble

    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    let mut regions = Vec::new();
    let mut region_grid: Grid<Option<Rc<RefCell<Region2>>>> =
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

            let current_region = Rc::new(RefCell::new(Region2 {
                name,
                area: Vec::new(),
                fences: 0,
                min_x: x,
                max_x: x,
                min_y: y,
                max_y: y,
            }));

            regions.push(current_region.clone());
            let mut current_region_cell = current_region.borrow_mut();

            while let Some(position) = queue.pop() {
                if region_grid.data[position.y][position.x].is_some() {
                    continue;
                }

                let (fences, neighbors) =
                    get_fences_and_neighbors(position, name, &grid, max_x, max_y);

                current_region_cell.area.push(position);
                current_region_cell.fences.add_assign(fences);
                current_region_cell.min_x = current_region_cell.min_x.min(position.x);
                current_region_cell.max_x = current_region_cell.max_x.max(position.x);
                current_region_cell.min_y = current_region_cell.min_y.min(position.y);
                current_region_cell.max_y = current_region_cell.max_y.max(position.y);

                neighbors.into_iter().for_each(|i| queue.push(i));

                region_grid.set_from_point(&position, Some(current_region.clone()));
            }
        }
    }

    // Solve
    let mut result = 0;
    for region in regions {
        result += count_sides(region.clone()) * region.borrow().area.len();
    }
    // Result
    println!("Result of part 2 is {}", result);
}

fn count_sides(region: Rc<RefCell<Region2>>) -> usize {
    let mut rtn = 0;
    let borrow = region.borrow();
    for position in borrow.area.iter() {
        let up = borrow.has(position.x, position.y.checked_sub(1).unwrap_or(usize::MAX));
        let down = borrow.has(position.x, position.y + 1);
        let left = borrow.has(position.x.checked_sub(1).unwrap_or(usize::MAX), position.y);
        let right = borrow.has(position.x + 1, position.y);

        let up_left = borrow.has(
            position.x.checked_sub(1).unwrap_or(usize::MAX),
            position.y.checked_sub(1).unwrap_or(usize::MAX),
        );
        let up_right = borrow.has(
            position.x + 1,
            position.y.checked_sub(1).unwrap_or(usize::MAX),
        );
        let down_left = borrow.has(
            position.x.checked_sub(1).unwrap_or(usize::MAX),
            position.y + 1,
        );
        let down_right = borrow.has(position.x + 1, position.y + 1);

        let border_count = 4 - [up, down, left, right].iter().filter(|&b| *b).count();

        let concave = [
            !up_left && up && left,
            !up_right && up && right,
            !down_right && down && right,
            !down_left && down && left,
        ]
        .iter()
        .filter(|&b| *b)
        .count();

        if border_count == 4 {
            rtn += 4 + concave;
            continue;
        }

        if border_count == 3 {
            rtn += 2 + concave;
            continue;
        }

        let convex = if border_count == 2 && !((up && down) || (left && right)) {
            1
        } else {
            0
        };

        rtn += concave + convex;
    }
    // println!("Region {} has {} sides", borrow.name, rtn);
    rtn
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
