use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;
use std::path::Path;

use utils::get_input_path;

enum ParserState {
    FirstLine,
    Empty,
    Rest,
}

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut patterns: Vec<String> = Vec::new();
    let mut parser_state = ParserState::FirstLine;
    let mut designs = Vec::new();

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        match parser_state {
            ParserState::FirstLine => {
                patterns = line
                    .split(", ")
                    .into_iter()
                    .map(|p| p.trim().to_string())
                    .collect();
                parser_state = ParserState::Empty;
            }
            ParserState::Empty => {
                parser_state = ParserState::Rest;
            }
            ParserState::Rest => {
                designs.push(line);
            }
        }
    }

    // Solve
    let mut result = 0;
    for design in &patterns {
        if is_design_valid(&design, &patterns) {
            result += 1;
        }
    }

    // let result = patterns
    //     .par_iter()
    //     .filter(|design| is_design_valid(&design, &patterns))
    //     .count();

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut patterns: Vec<String> = Vec::new();
    let mut parser_state = ParserState::FirstLine;
    let mut designs = Vec::new();

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        match parser_state {
            ParserState::FirstLine => {
                patterns = line
                    .split(", ")
                    .into_iter()
                    .map(|p| p.trim().to_string())
                    .collect();
                parser_state = ParserState::Empty;
            }
            ParserState::Empty => {
                parser_state = ParserState::Rest;
            }
            ParserState::Rest => {
                designs.push(line);
            }
        }
    }

    // Solve
    // let mut result = 0;
    // for design in designs {
    //     result += get_number_of_valid_designs(&design, &patterns);
    // }

    let result: usize = designs
        .par_iter()
        .map(|x| get_number_of_valid_designs(x, &patterns))
        .sum();

    // let mut memo = HashMap::new();
    // let mut result = 0;
    // for design in &designs {
    //     result += count_possible(design, &patterns, &mut memo);
    // }

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

fn is_design_valid(design: &String, patterns: &Vec<String>) -> bool {
    let mut queue: BTreeSet<usize> = BTreeSet::new();

    queue.insert(0);

    while let Some(position) = queue.pop_last() {
        if design.len() == position {
            return true;
        }
        let string_start = &design[position..];

        for pattern in patterns {
            if string_start.starts_with(pattern) {
                queue.insert(position + pattern.len());
            }
        }
    }
    false
}

fn get_number_of_valid_designs(design: &String, patterns: &Vec<String>) -> usize {
    let mut queue: BTreeMap<usize, usize> = BTreeMap::new();

    queue.insert(0, 1);

    let mut rtn = 0;

    while let Some((position, count)) = queue.pop_first() {
        if design.len() == position {
            rtn += count;
        }
        let string_start = &design[position..];

        for pattern in patterns {
            if string_start.starts_with(pattern) {
                queue
                    .entry(position + pattern.len())
                    .or_default()
                    .add_assign(count);
            }
        }
    }
    rtn
}

// fn count_possible<'a>(pattern: &'a str, towels: &Vec<String>, memo: &mut HashMap<&'a str, u64>) -> u64 {
//     if let Some(&c) = memo.get(pattern) { return c; }
//
//     if pattern.trim().is_empty() { return 1; }
//
//     let mut count = 0;
//     for towel in towels {
//         if pattern.starts_with(towel) {
//             count += count_possible(&pattern[towel.len()..], &towels, memo);
//         }
//     }
//
//     *memo.entry(pattern).or_insert(0) += count;
//     count
// }
