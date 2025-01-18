use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use utils::get_input_path;

fn run(input_file: &Path) {
    // Preamble
    const ROBOT_KEYPADS: usize = 2;
    let mut codes = Vec::new();
    let file = File::open(input_file).unwrap();
    let mut paths = Vec::new();
    let directional_key_costs = calc_directional_key_costs::<ROBOT_KEYPADS>();

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        codes.push(line);
    }

    // Solve
    let mut results = Vec::with_capacity(codes.len());
    for code in &codes {
        let mut results_per_char = 0;
        let mut pos = NUMERIC_KEY_POSITIONS.len() - 1;
        for c in code.chars() {
            let new_pos = match c {
                '0'..='9' => c as usize - '0' as usize,
                'A' => 10,
                _ => panic!("Invalid character: {}", c),
            };
            get_paths::<3>(&mut paths, &NUMERIC_KEY_POSITIONS, pos, new_pos);
            let cost: usize = paths
                .iter()
                .map(|path| {
                    let mut pos = DirectionKey::Activate;
                    path.iter()
                        .map(|&new_pos| {
                            let cost = directional_key_costs[pos as usize * 5 + new_pos as usize];
                            pos = new_pos;
                            cost
                        })
                        .sum()
                })
                .min()
                .unwrap();
            pos = new_pos;
            paths.clear();
            results_per_char += cost;
        }
        results.push(results_per_char);
    }

    // Result
    let mut result = 0;

    for (i, code) in codes.iter().enumerate() {
        let mut code_chars_itr = code.chars();
        let number = code_chars_itr.next().unwrap().to_digit(10).unwrap() * 100
            + code_chars_itr.next().unwrap().to_digit(10).unwrap() * 10
            + code_chars_itr.next().unwrap().to_digit(10).unwrap();

        result += results[i] * number as usize;
    }

    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    const ROBOT_KEYPADS: usize = 25;
    let mut codes = Vec::new();
    let file = File::open(input_file).unwrap();
    let mut paths = Vec::new();
    let directional_key_costs = calc_directional_key_costs::<ROBOT_KEYPADS>();

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        codes.push(line);
    }

    // Solve
    let mut results = Vec::with_capacity(codes.len());
    for code in &codes {
        let mut results_per_char = 0;
        let mut pos = NUMERIC_KEY_POSITIONS.len() - 1;
        for c in code.chars() {
            let new_pos = match c {
                '0'..='9' => c as usize - '0' as usize,
                'A' => 10,
                _ => panic!("Invalid character: {}", c),
            };
            get_paths::<3>(&mut paths, &NUMERIC_KEY_POSITIONS, pos, new_pos);
            let cost: usize = paths
                .iter()
                .map(|path| {
                    let mut pos = DirectionKey::Activate;
                    path.iter()
                        .map(|&new_pos| {
                            let cost = directional_key_costs[pos as usize * 5 + new_pos as usize];
                            pos = new_pos;
                            cost
                        })
                        .sum()
                })
                .min()
                .unwrap();
            pos = new_pos;
            paths.clear();
            results_per_char += cost;
        }
        results.push(results_per_char);
    }

    // Result
    let mut result = 0;

    for (i, code) in codes.iter().enumerate() {
        let mut code_chars_itr = code.chars();
        let number = code_chars_itr.next().unwrap().to_digit(10).unwrap() * 100
            + code_chars_itr.next().unwrap().to_digit(10).unwrap() * 10
            + code_chars_itr.next().unwrap().to_digit(10).unwrap();

        result += results[i] * number as usize;
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

const NUMERIC_KEY_POSITIONS: [[usize; 2]; 11] = [
    [1, 3],
    [0, 2],
    [1, 2],
    [2, 2],
    [0, 1],
    [1, 1],
    [2, 1],
    [0, 0],
    [1, 0],
    [2, 0],
    [2, 3],
];

#[derive(Debug, Clone, Copy)]
enum DirectionKey {
    Up = 0,
    Activate = 1,
    Left = 2,
    Down = 3,
    Right = 4,
}

fn get_paths<const HOLE_Y: usize>(
    paths: &mut Vec<Vec<DirectionKey>>,
    key_positions: &[[usize; 2]],
    start: usize,
    end: usize,
) {
    let [start_x, start_y] = key_positions[start];
    let [end_x, end_y] = key_positions[end];

    if !(start_x == 0 && end_y == HOLE_Y) {
        // Start by going vertically and then horizontally
        // This must not be done if we start on the left button and go to the top row, as that would make us pass
        // over an empty space.
        let mut path = Vec::new();
        if start_y < end_y {
            path.extend((start_y..end_y).map(|_| DirectionKey::Down));
        } else if start_y > end_y {
            path.extend((end_y..start_y).map(|_| DirectionKey::Up));
        }
        if start_x < end_x {
            path.extend((start_x..end_x).map(|_| DirectionKey::Right));
        } else if start_x > end_x {
            path.extend((end_x..start_x).map(|_| DirectionKey::Left));
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(DirectionKey::Activate);
        paths.push(path);
    }

    if start_x != end_x && start_y != end_y && !(start_y == HOLE_Y && end_x == 0) {
        // If we need to both vertically and horizontally, we can also do it by going horizontally first.
        // This must not be done if we end on the left button, as that would make us pass over an empty space.
        let mut path = Vec::new();
        if start_x < end_x {
            path.extend((start_x..end_x).map(|_| DirectionKey::Right));
        } else if start_x > end_x {
            path.extend((end_x..start_x).map(|_| DirectionKey::Left));
        }
        if start_y < end_y {
            path.extend((start_y..end_y).map(|_| DirectionKey::Down));
        } else if start_y > end_y {
            path.extend((end_y..start_y).map(|_| DirectionKey::Up));
        }
        // We always need to end with Activate, so we actually press the button we go to.
        path.push(DirectionKey::Activate);
        paths.push(path);
    }

    // It is never worth zigzagging, as such paths can be reduced into a non-zigzagging path just by duplicating
    // some presses while eliminating others, to get a path that takes fewer presses in total.
}

// Where each key is located on the directional keypad
const DIRECTION_KEY_POSITIONS: [[usize; 2]; 5] = [[1, 0], [2, 0], [0, 1], [1, 1], [2, 1]];

fn calc_directional_key_costs<const ROBOT_KEYPADS: usize>() -> Vec<usize> {
    // Possible button inputs required to get the robot at the next level to press any button from any starting position
    let direction_key_paths: Vec<_> = (0..(5 * 5))
        .map(|i| {
            let mut paths = Vec::new();
            let start = i / 5;
            let end = i % 5;
            get_paths::<0>(&mut paths, &DIRECTION_KEY_POSITIONS, start, end);
            paths
        })
        .collect();

    // How many button presses it takes to get to any button from any other button and then press it
    let mut path_costs: Vec<usize> = direction_key_paths
        .iter()
        .map(|paths| paths.iter().map(|path| path.len()).min().unwrap())
        .collect();

    for _ in 0..ROBOT_KEYPADS - 1 {
        path_costs = calc_level_costs(&path_costs, &direction_key_paths);
    }

    path_costs
}

fn calc_level_costs(previous_costs: &[usize], paths: &[Vec<Vec<DirectionKey>>]) -> Vec<usize> {
    paths
        .iter()
        .map(|paths| {
            paths
                .iter()
                .map(|path| {
                    // Sum up the costs of going from each button to the next one and pressing it, starting from Activate
                    let mut pos = DirectionKey::Activate;
                    path.iter()
                        .map(|&new_pos| {
                            let cost = previous_costs[pos as usize * 5 + new_pos as usize];
                            pos = new_pos;
                            cost
                        })
                        .sum()
                })
                .min()
                .unwrap()
        })
        .collect()
}
