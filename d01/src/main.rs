use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;
use std::path::Path;

use utils::get_input_path;

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    // Parse
    let reader = BufReader::new(file);
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut numbers = line.split_ascii_whitespace();
        let left_number: usize = numbers.next().unwrap().parse().unwrap();
        let right_number: usize = numbers.next().unwrap().parse().unwrap();

        left.push(left_number);
        right.push(right_number)
    }

    // Solve
    left.sort_unstable();
    right.sort_unstable();
    let mut result = 0;

    for i in 0..left.len() {
        let distance = left[i].abs_diff(right[i]);
        result += distance
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    // Parse
    let reader = BufReader::new(file);
    let mut left = Vec::new();
    let mut right = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut numbers = line.split_ascii_whitespace();
        let left_number: usize = numbers.next().unwrap().parse().unwrap();
        let right_number: usize = numbers.next().unwrap().parse().unwrap();

        left.push(left_number);
        let value = right.entry(right_number).or_insert(0_usize);
        value.add_assign(1);
    }

    // Solve
    // dbg!(left);
    // dbg!(right);
    let mut result = 0;

    for n in left {
        let appearances = right.get(&n).unwrap_or(&0);
        let similarity_score = n * appearances;
        result += similarity_score;
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
