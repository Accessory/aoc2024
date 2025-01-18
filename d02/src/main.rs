use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use utils::get_input_path;

#[derive(Eq, PartialEq)]
enum Direction {
    Ascending,
    Descending,
    Unset,
}

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    // Parse
    let reader = BufReader::new(file);
    let mut reports = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let numbers_as_string = line.split_ascii_whitespace();
        let report: Vec<i32> = numbers_as_string
            .into_iter()
            .map(|n| n.parse().unwrap())
            .collect();
        reports.push(report)
    }

    // Solve
    let mut result = 0;

    'outer: for report in reports {
        let mut direction = Direction::Unset;
        for window in report.windows(2) {
            let distance = window[0] - window[1];
            let current_direction = match distance {
                -3 => Direction::Ascending,
                -2 => Direction::Ascending,
                -1 => Direction::Ascending,
                1 => Direction::Descending,
                2 => Direction::Descending,
                3 => Direction::Descending,
                _ => continue 'outer,
            };

            if direction == Direction::Unset {
                direction = current_direction;
                continue;
            }

            if direction != current_direction {
                continue 'outer;
            }
        }
        result += 1;
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    // Parse
    let reader = BufReader::new(file);
    let mut reports = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let numbers_as_string = line.split_ascii_whitespace();
        let report: Vec<i32> = numbers_as_string
            .into_iter()
            .map(|n| n.parse().unwrap())
            .collect();
        reports.push(report)
    }

    // Solve
    let mut result = 0;

    for report in reports {
        let mut success = false;
        'next: for skip in (0..report.len()).rev() {
            let mut direction = Direction::Unset;
            for i in 0..report.len() - 1 {
                let current_i = if i == skip { i + 1 } else { i };
                let mut next = current_i + 1;
                if next == skip {
                    next += 1;
                }
                if next >= report.len() {
                    break;
                }
                let distance = report[current_i] - report[next];
                let current_direction = match distance {
                    -3 => Direction::Ascending,
                    -2 => Direction::Ascending,
                    -1 => Direction::Ascending,
                    1 => Direction::Descending,
                    2 => Direction::Descending,
                    3 => Direction::Descending,
                    _ => {
                        // println!("Next at position {} {}",report[current_i], report[next]);
                        continue 'next;
                    }
                };

                if direction == Direction::Unset {
                    direction = current_direction;
                    continue;
                }

                if direction != current_direction {
                    continue 'next;
                }
            }
            success = true;
            break;
        }
        if success {
            // println!("Report  \"{:?}\" is ok", report);
            result += 1;
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
