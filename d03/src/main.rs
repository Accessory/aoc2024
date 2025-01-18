use regex::Regex;
use std::fs;
use std::path::Path;
use utils::get_input_path;

fn run(input_file: &Path) {
    // Preamble
    let mut result: usize = 0;
    let rgx = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let capture_matches = rgx.captures_iter(line.as_str());

    // Solve
    for matches in capture_matches {
        let left: usize = matches.get(1).unwrap().as_str().parse().unwrap();
        let right: usize = matches.get(2).unwrap().as_str().parse().unwrap();
        // println!("Found mul {left}, {right}");
        result += left * right
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    let mut result: usize = 0;
    let rgx = Regex::new(r#"mul\((\d+),(\d+)\)|don\'t\(\)|do\(\)"#).unwrap();

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let capture_matches = rgx.captures_iter(line.as_str());

    // Solve
    let mut is_active = true;
    for matches in capture_matches {
        let full_text = matches.get(0).unwrap().as_str();
        match full_text {
            "do()" => {
                is_active = true;
            }
            "don't()" => {
                is_active = false;
            }
            _ => {
                if is_active {
                    let left: usize = matches.get(1).unwrap().as_str().parse().unwrap();
                    let right: usize = matches.get(2).unwrap().as_str().parse().unwrap();
                    // println!("Found mul {left}, {right}");
                    result += left * right
                }
            }
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
