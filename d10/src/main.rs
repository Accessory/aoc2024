use std::path::Path;

use utils::grid::Grid;
use utils::grid_point::GridPoint;
use utils::{get_input_path, parse_into_u8_vector_vector};

struct TrailContext {
    high: u8,
    position: GridPoint,
}

fn run(input_file: &Path) {
    // Preamble
    // Parse
    let grid: Grid<u8> = parse_into_u8_vector_vector(input_file).into();
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    let mut starts = Vec::new();

    // Prepare
    for y in 0..max_y {
        for x in 0..max_x {
            if grid.data[y][x] == 0 {
                starts.push(GridPoint { x, y })
            }
        }
    }

    let mut trail_results = Vec::new();

    // Solve
    for start in starts {
        let mut trail_result: Vec<GridPoint> = Vec::new();
        let mut queue = Vec::new();
        queue.push(TrailContext {
            high: 0,
            position: start,
        });

        while let Some(current) = queue.pop() {
            if current.high == 9 {
                if !trail_result.contains(&current.position) {
                    trail_result.push(current.position);
                }
                continue;
            }

            let next = current.high + 1;

            for neighbor in current
                .position
                .generate_non_diagonal_neighbors_with_check(max_x, max_y)
            {
                let neighbor_height = grid.data[neighbor.y][neighbor.x];
                if neighbor_height == next {
                    queue.push(TrailContext {
                        high: next,
                        position: neighbor,
                    })
                }
            }
        }
        trail_results.push(trail_result.len());
    }

    // Result
    let result: usize = trail_results.iter().sum();

    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    // Parse
    let grid: Grid<u8> = parse_into_u8_vector_vector(input_file).into();
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();

    let mut starts = Vec::new();

    // Prepare
    for y in 0..max_y {
        for x in 0..max_x {
            if grid.data[y][x] == 0 {
                starts.push(GridPoint { x, y })
            }
        }
    }

    let mut trail_results = Vec::new();

    // Solve
    for start in starts {
        let mut trail_result: usize = 0;
        let mut queue = Vec::new();
        queue.push(TrailContext {
            high: 0,
            position: start,
        });

        while let Some(current) = queue.pop() {
            if current.high == 9 {
                trail_result += 1;
                continue;
            }

            let next = current.high + 1;

            for neighbor in current
                .position
                .generate_non_diagonal_neighbors_with_check(max_x, max_y)
            {
                let neighbor_height = grid.data[neighbor.y][neighbor.x];
                if neighbor_height == next {
                    queue.push(TrailContext {
                        high: next,
                        position: neighbor,
                    })
                }
            }
        }
        trail_results.push(trail_result);
    }

    // Result
    let result: usize = trail_results.iter().sum();

    println!("Result of part 2 is {}", result);
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
