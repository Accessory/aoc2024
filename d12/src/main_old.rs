use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::AddAssign;
use std::path::Path;
use std::rc::Rc;
use utils::grid::Grid;
use utils::grid_direction::GridDirection;
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
    // Holes
    let mut seen = HashMap::new();
    let mut holes = Vec::new();
    for y in region.borrow().min_y + 1..region.borrow().max_y {
        for x in region.borrow().min_x + 1..region.borrow().max_x {
            let position = GridPoint { x, y };
            if !region.borrow().area.contains(&position) {
                if !seen.contains_key(&position) && is_hole(region.clone(), position, &mut seen) {
                    dbg!(&region);
                    // dbg!(&seen);
                    holes.push(position);
                }
            }
        }
    }

    println!("Region {} has {} holes", region.borrow().name, holes.len());

    let mut points: HashMap<GridPoint, Vec<GridDirection>> = HashMap::new();
    let max_x = region.borrow().max_x + 1;
    let max_y = region.borrow().max_y + 1;

    // Graph
    for area in region.borrow().area.iter() {
        let tl = points.entry(*area).or_default();
        tl.push(GridDirection::Right);
        tl.push(GridDirection::Down);
        let tr = points.entry(area.next_right()).or_default();
        tr.push(GridDirection::Down);
        tr.push(GridDirection::Left);
        let dl = points.entry(area.next_down()).or_default();
        dl.push(GridDirection::Up);
        dl.push(GridDirection::Right);
        let dr = points
            .entry(GridPoint::new(area.x + 1, area.y + 1))
            .or_default();
        dr.push(GridDirection::Up);
        dr.push(GridDirection::Left);
    }

    let start = *region.borrow().area.first().unwrap();
    holes.push(start);

    // Find boarders

    let mut rtn:usize = 0;
    for s in holes {
        rtn += get_boarders_count(s, &points, max_x, max_y)
    }
    println!("Region {} has {} sides", region.borrow().name, rtn);
    rtn
}

fn get_boarders_count(start: GridPoint, points: &HashMap<GridPoint, Vec<GridDirection>>, max_x: usize, max_y: usize) -> usize {
    let mut direction = GridDirection::Up;

    let mut direction_list = Vec::with_capacity(points.len());
    let mut points_list = Vec::with_capacity(points.len());

    let mut position = start;
    loop {
        let current_point = points.get(&position).unwrap();
        let next_list = match direction {
            GridDirection::Up => [GridDirection::Left, GridDirection::Up, GridDirection::Right],
            GridDirection::Right => [GridDirection::Up, GridDirection::Right, GridDirection::Down],
            GridDirection::Down => [
                GridDirection::Right,
                GridDirection::Down,
                GridDirection::Left,
            ],
            GridDirection::Left => [GridDirection::Down, GridDirection::Left, GridDirection::Up],
        };

        // points_list.push(position);
        // direction_list.push(direction);

        for next in next_list.iter().filter(|&p| current_point.contains(p)) {
            if let Some(next_position) = position.next_by_direction_with_check(&next, max_x, max_y)
            {
                // if let Some(point) = points.get(&next_position) {
                direction_list.push(*next);
                points_list.push(next_position);
                direction = *next;
                position = next_position;
                break;
                // }
            }
        }

        if position == start {
            break;
        }
    }

    // dbg!(points_list);
    // dbg!(&direction_list);

    let mut rtn = 1;
    for window in direction_list.windows(2) {
        if window[0] != window[1] {
            rtn += 1;
        }
    }

    rtn
}

fn is_hole(region: Rc<RefCell<Region2>>, start: GridPoint, seen: &mut HashMap<GridPoint, bool>) -> bool {
    let mut queue = vec![start];

    let min_x = region.borrow().min_x;
    let min_y = region.borrow().min_y;
    let max_x = region.borrow().max_x;
    let max_y = region.borrow().max_y;

    let mut inner_seen = HashSet::new();

    let mut rtn = true;

    while let Some(item) = queue.pop() {
        if let Some(rtn) = seen.get(&item) {
            return *rtn;
        }

        if !inner_seen.insert(item) {
            continue;
        }
        if item.x <= min_x || item.x >= max_x || item.y <= min_y || item.y >= max_y {
            rtn = false;
        }

        for neighbor in item.generate_non_diagonal_neighbors_with_check(max_x + 2, max_y + 2) {
            if !region.borrow().area.contains(&neighbor) {
                queue.push(neighbor);
            }
        }
    }

    inner_seen.into_iter().for_each(|t| { seen.insert(t, rtn); });

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
