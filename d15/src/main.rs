use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use utils::get_input_path;
use utils::grid::Grid;
use utils::grid_direction::GridDirection;
use utils::grid_point::GridPoint;

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();

    // Parse
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<GridDirection> = Vec::new();
    let mut is_map = true;
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.is_empty() {
            if is_map {
                is_map = false;
            } else {
                break;
            }
        }

        if is_map {
            let row: Vec<char> = line.chars().collect();
            map.push(row);
        } else {
            line.chars().for_each(|c| moves.push((&c).into()));
        }
    }

    // Prepare
    let mut grid: Grid<char> = map.into();
    let mut robot = grid.find_first(&'@').unwrap();

    // Solve
    // println!("Initial state:");
    // grid.print_data();
    // println!();

    for m in moves {
        robot = do_move(robot, &mut grid, m);

        // println!("Move {}:", m);
        // grid.print_data();
        // println!();
    }

    // Result
    // println!("Final state:");
    // grid.print_data();
    let mut result = 0;

    for y in 0..grid.get_max_y() {
        for x in 0..grid.get_max_x() {
            if grid.data[y][x] == 'O' {
                result += 100 * y + x;
            }
        }
    }

    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();

    // Parse
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<GridDirection> = Vec::new();
    let mut is_map = true;
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.is_empty() {
            if is_map {
                is_map = false;
            } else {
                break;
            }
        }

        if is_map {
            let mut row: Vec<char> = Vec::with_capacity(line.len() * 2);

            for c in line.chars() {
                match c {
                    '#' => {
                        row.push('#');
                        row.push('#');
                    }
                    '.' => {
                        row.push('.');
                        row.push('.');
                    }
                    'O' => {
                        row.push('[');
                        row.push(']');
                    }
                    '@' => {
                        row.push('@');
                        row.push('.');
                    }
                    _ => panic!("Should not be here!"),
                }
            }

            map.push(row);
        } else {
            line.chars().for_each(|c| moves.push((&c).into()));
        }
    }

    // Prepare
    let mut grid: Grid<char> = map.into();
    let mut robot = grid.find_first(&'@').unwrap();

    // Solve
    // println!("Initial state:");
    // grid.print_data();
    // println!();

    for m in moves {
        robot = do_move_2(robot, &mut grid, m);

        // if idx == 21 {
        // println!("Move {} after {} rounds:", m, idx + 1);
        // grid.print_data();
        // println!();
        // break;
        // }
    }

    // Result
    // println!("Final state:");
    // grid.print_data();
    let mut result = 0;

    for y in 0..grid.get_max_y() {
        for x in 0..grid.get_max_x() {
            if grid.data[y][x] == '[' {
                result += 100 * y + x;
            }
        }
    }

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

#[derive(Debug)]
struct Doings {
    from: GridPoint,
    to: GridPoint,
}

impl Doings {
    fn new(from: GridPoint, to: GridPoint) -> Self {
        Self { from, to }
    }
}

fn do_move_2(robot: GridPoint, grid: &mut Grid<char>, direction: GridDirection) -> GridPoint {
    let mut to_dos = Vec::new();
    let mut seen = HashSet::new();

    let mut queue: VecDeque<(GridPoint, GridPoint)> = VecDeque::new();
    queue.push_back((robot, robot.next_by_direction(&direction)));

    while let Some((prev, next)) = queue.pop_front() {
        if !seen.insert(prev) {
            continue;
        }
        match direction {
            GridDirection::Up | GridDirection::Down => match grid.data[next.y][next.x] {
                '#' => {
                    return robot;
                }
                '[' => {
                    to_dos.push(Doings::new(prev, next));
                    queue.push_back((next, next.next_by_direction(&direction)));
                    let next_other = next.next_right();
                    queue.push_back((next_other, next_other.next_by_direction(&direction)));
                }
                ']' => {
                    to_dos.push(Doings::new(prev, next));
                    queue.push_back((next, next.next_by_direction(&direction)));
                    let next_other = next.next_left();
                    queue.push_back((next_other, next_other.next_by_direction(&direction)));
                }
                '.' => {
                    to_dos.push(Doings::new(prev, next));
                }
                _ => {
                    panic!(
                        "Should not be here. The current Char is {}",
                        grid.data[next.y][next.x]
                    );
                }
            },
            GridDirection::Right | GridDirection::Left => match grid.data[next.y][next.x] {
                '#' => {
                    return robot;
                }
                '[' | ']' => {
                    to_dos.push(Doings::new(prev, next));
                    queue.push_back((next, next.next_by_direction(&direction)));
                }
                '.' => {
                    to_dos.push(Doings::new(prev, next));
                }
                _ => {
                    panic!(
                        "Should not be here. The current Char is {}",
                        grid.data[next.y][next.x]
                    );
                }
            },
        }
    }

    while let Some(to_do) = to_dos.pop() {
        let to_char = grid.data[to_do.to.y][to_do.to.x];
        let from_char = grid.data[to_do.from.y][to_do.from.x];
        grid.set_from_point(&to_do.to, from_char);
        grid.set_from_point(&to_do.from, to_char);
        // grid.print_data();
        // println!()
    }

    robot.next_by_direction(&direction)
}

fn do_move(mut robot: GridPoint, grid: &mut Grid<char>, direction: GridDirection) -> GridPoint {
    let mut positions = vec![];
    let mut current = robot.next_by_direction(&direction);
    loop {
        match grid.data[current.y][current.x] {
            '#' => return robot,
            'O' => {
                positions.push(current);
                current = current.next_by_direction(&direction);
            }
            '.' => {
                grid.set_from_point(&robot, '.');
                let mut is_first = true;
                positions.push(current);
                for current_box_position in positions {
                    if is_first {
                        grid.set_from_point(&current_box_position, '@');
                        robot = current_box_position;
                        is_first = false;
                    } else {
                        grid.set_from_point(&current_box_position, 'O');
                    }
                }
                return robot;
            }
            _ => {
                panic!(
                    "Should not be here! The current input is {}",
                    grid.data[current.y][current.x]
                );
            }
        }
    }
}
