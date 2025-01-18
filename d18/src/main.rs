use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use utils::get_input_path;
use utils::grid::Grid;
use utils::grid_point::GridPoint;

#[cfg(test)]
const SPACE_X: usize = 7;
#[cfg(test)]
const SPACE_Y: usize = 7;

#[cfg(not(test))]
const SPACE_X: usize = 71;
#[cfg(not(test))]
const SPACE_Y: usize = 71;

#[derive(Clone, Debug, Default)]
struct SearchContext {
    position: GridPoint,
    previous: Vec<GridPoint>,
    steps: usize,
}

fn run(input_file: &Path) {
    #[cfg(test)]
    const USED_BYTES: usize = 12;
    #[cfg(not(test))]
    const USED_BYTES: usize = 1024;

    // let start = GridPoint::new(0, 0);
    let end = GridPoint::new(SPACE_X - 1, SPACE_Y - 1);

    // Preamble
    let mut grid: Grid<char> = Grid::with_width_height(SPACE_X, SPACE_Y, '.');
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let file = File::open(input_file).unwrap();
    let mut bytes: Vec<GridPoint> = Vec::new();

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split(",");
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        bytes.push(GridPoint { x, y })
    }

    for byte_coords in &bytes.as_slice()[0..USED_BYTES] {
        grid.set_from_point(byte_coords, '#');
    }

    let mut queue: VecDeque<SearchContext> = VecDeque::new();
    queue.push_back(SearchContext::default());

    let mut seen = HashSet::new();

    let mut winning_context = None;

    while let Some(mut context) = queue.pop_front() {
        if context.position == end {
            winning_context = Some(context);
            break;
        }

        if grid.data[context.position.y][context.position.x] == '#' {
            continue;
        }

        if !seen.insert(context.position) {
            continue;
        }

        let neighbors = context
            .position
            .generate_non_diagonal_neighbors_with_check(max_x, max_y);

        let next_step = context.steps + 1;
        context.previous.push(context.position);

        for neighbor in neighbors {
            queue.push_back(SearchContext {
                position: neighbor,
                previous: context.previous.clone(),
                steps: next_step,
            })
        }
    }

    // grid.print_data();

    // Solve
    let result = winning_context.unwrap().steps;
    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    #[cfg(test)]
    const USED_BYTES: usize = 12;
    #[cfg(not(test))]
    const USED_BYTES: usize = 1024;

    // let start = GridPoint::new(0, 0);
    let end = GridPoint::new(SPACE_X - 1, SPACE_Y - 1);

    // Preamble
    let mut grid: Grid<char> = Grid::with_width_height(SPACE_X, SPACE_Y, '.');
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let file = File::open(input_file).unwrap();
    let mut bytes: Vec<GridPoint> = Vec::new();

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split(",");
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        bytes.push(GridPoint { x, y })
    }

    let mut last_ub = 0;
    let mut last_winning_path = Vec::new();
    let mut auto_ok = false;
    for ub in USED_BYTES + 1..bytes.iter().len() {
        for byte_coords in &bytes.as_slice()[last_ub..ub] {
            // println!("Add # at {},{}", byte_coords.x, byte_coords.y);
            grid.set_from_point(byte_coords, '#');
            auto_ok = auto_ok && !last_winning_path.contains(byte_coords);
        }
        last_ub = ub;

        if auto_ok {
            continue;
        }

        let mut queue: VecDeque<SearchContext> = VecDeque::new();
        queue.push_back(SearchContext::default());

        let mut seen = HashSet::new();
        let mut winning_context = None;

        while let Some(mut context) = queue.pop_front() {
            if context.position == end {
                winning_context = Some(context);
                break;
            }

            if grid.data[context.position.y][context.position.x] == '#' {
                continue;
            }

            if !seen.insert(context.position) {
                continue;
            }

            let neighbors = context
                .position
                .generate_non_diagonal_neighbors_with_check(max_x, max_y);

            let next_step = context.steps + 1;
            context.previous.push(context.position);

            for neighbor in neighbors {
                queue.push_back(SearchContext {
                    position: neighbor,
                    previous: context.previous.clone(),
                    steps: next_step,
                })
            }
        }
        if winning_context.is_none() {
            break;
        }
        last_winning_path = winning_context.unwrap().previous;
        auto_ok = true;
    }

    // grid.print_data();

    // Solve
    let result = bytes[last_ub - 1];
    // Result
    println!("Result of part 2 is {},{}", result.x, result.y);
}

fn main() {
    let input_file = get_input_path(env!("CARGO_MANIFEST_DIR"));

    println!("Running {}", env!("CARGO_PKG_NAME"));
    println!("InputFile: {}", input_file.display());

    run(input_file.as_path());
    run2(input_file.as_path());
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        run(&get_test_input_path(env!("CARGO_MANIFEST_DIR")));
    }

    #[test]
    fn test_input_part_2() {
        run2(&get_test_input_path(env!("CARGO_MANIFEST_DIR")));
    }
}
