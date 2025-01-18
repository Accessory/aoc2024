use std::ops::AddAssign;
use std::path::Path;

use utils::grid::Grid;
use utils::grid_direction::Grid8WayDirection;
use utils::grid_point::GridPoint;
use utils::{get_input_path, parse_into_char_vector_vector};

fn run(input_file: &Path) {
    // Preamble
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    // Solve
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let mut result = 0;

    for x in 0..max_x {
        'next_y: for y in 0..max_y {
            let start = GridPoint { x, y };
            'next_direction: for direction in Grid8WayDirection::get_all_directions_array() {
                if grid.get_from_point(&start).is_none_or(|&c| c != XMAS[0]) {
                    continue 'next_y;
                }
                let mut current_position = start;
                for word_position in 1..XMAS.len() {
                    current_position = match current_position
                        .next_by_8direction_with_check(&direction, max_x, max_y)
                    {
                        None => continue 'next_direction,
                        Some(pos) => pos,
                    };
                    if grid
                        .get_from_point(&current_position)
                        .is_none_or(|&c| c != XMAS[word_position])
                    {
                        continue 'next_direction;
                    }
                    // println!("Current position: x: {} y: {} Current Direction: {}", current_position.x, current_position.y, direction);
                }
                result += 1;
            }
        }
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    // Solve
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let mut result = 0;

    for x in 1..max_x - 1 {
        'next_y: for y in 1..max_y - 1 {
            let start = GridPoint { x, y };

            if grid.get_from_point(&start).is_none_or(|&c| c != 'A') {
                continue 'next_y;
            }
            let mut left_up: Option<char> = None;
            let mut right_up: Option<char> = None;
            for check in [
                Grid8WayDirection::LeftUp,
                Grid8WayDirection::RightUp,
                Grid8WayDirection::RightDown,
                Grid8WayDirection::LeftDown,
            ] {
                let to_check = start
                    .next_by_8direction_with_check(&check, max_x, max_y)
                    .unwrap();
                let c = grid.get_from_point(&to_check).unwrap();
                match c {
                    'M' => {}
                    'S' => {}
                    _ => continue 'next_y,
                }
                match check {
                    Grid8WayDirection::LeftUp => {
                        left_up = Some(*c);
                    }
                    Grid8WayDirection::RightUp => {
                        right_up = Some(*c);
                    }
                    Grid8WayDirection::RightDown => {
                        if left_up.is_some_and(|lu| &lu == c) {
                            continue 'next_y;
                        }
                    }
                    Grid8WayDirection::LeftDown => {
                        if right_up.is_some_and(|lu| &lu == c) {
                            continue 'next_y;
                        }
                    }
                    _ => {}
                }
            }
            result.add_assign(1);
        }
    }

    // Result
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
