use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Div;
use std::path::Path;

use utils::get_input_path;

enum ParserState {
    Rules,
    PrintingOrders,
}

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut printings: Vec<Vec<usize>> = Vec::new();
    let mut before_after: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut after_before: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    // Parse
    let reader = BufReader::new(file);

    let mut current_parsing_state = ParserState::Rules;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        match current_parsing_state {
            ParserState::Rules => {
                if line.is_empty() {
                    current_parsing_state = ParserState::PrintingOrders;
                    continue;
                }
                let mut split = line.split("|");
                let before = split.next().unwrap().parse().unwrap();
                let after = split.next().unwrap().parse().unwrap();
                before_after.entry(before).or_default().push(after);
                after_before.entry(after).or_default().push(before);
            }
            ParserState::PrintingOrders => {
                let numbers: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
                printings.push(numbers);
            }
        }
    }

    // Solve
    let mut result = 0;
    // let mut successes = 0;
    for printing in printings {
        if validate_ordering(&printing, &before_after, &after_before) {
            let middle = printing.len().div(2);
            // println!("Row {:?}", printing);
            // println!("Middle: {middle} - {}", printing[middle]);
            result += printing[middle];
            // successes += 1;
        }
    }

    // Result
    // println!("Successes {}", successes);
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut printings: Vec<Vec<usize>> = Vec::new();
    let mut before_after: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut after_before: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    // Parse
    let reader = BufReader::new(file);

    let mut current_parsing_state = ParserState::Rules;

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        match current_parsing_state {
            ParserState::Rules => {
                if line.is_empty() {
                    current_parsing_state = ParserState::PrintingOrders;
                    continue;
                }
                let mut split = line.split("|");
                let before = split.next().unwrap().parse().unwrap();
                let after = split.next().unwrap().parse().unwrap();
                before_after.entry(before).or_default().push(after);
                after_before.entry(after).or_default().push(before);
            }
            ParserState::PrintingOrders => {
                let numbers: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
                printings.push(numbers);
            }
        }
    }

    // Solve
    let mut result = 0;
    for mut printing in printings {
        if !validate_ordering(&printing, &before_after, &after_before) {
            // print!("From: {:?}", printing);
            printing = order_printing(printing, &before_after);
            // println!(" To: {:?}", printing);
            let middle = printing.len().div(2);
            result += printing[middle];
        }
    }

    // Result
    println!("Result of part 2 is {}", result);
}

fn order_printing(
    mut printing: Vec<usize>,
    before_after: &BTreeMap<usize, Vec<usize>>,
    // after_before: &HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
    printing.sort_unstable_by(|l, r| {
        if let Some(after) = before_after.get(l) {
            if after.contains(r) {
                return Ordering::Less;
            }
        }
        // if let Some(before) = after_before.get(r) {
        //     if before.contains(l) {
        //         return Ordering::Greater;
        //     }
        // }
        Ordering::Greater
    });

    printing
}

fn validate_ordering(
    printing: &[usize],
    before_after: &BTreeMap<usize, Vec<usize>>,
    after_before: &BTreeMap<usize, Vec<usize>>,
) -> bool {
    for i in 0..printing.len() {
        let current = printing[i];

        // Before
        let left = &printing[0..i];
        if let Some(after) = before_after.get(&current) {
            for a in after {
                if left.contains(a) {
                    return false;
                }
            }
        }

        // After
        let right = &printing[i + 1..printing.len()];
        if let Some(before) = after_before.get(&current) {
            for b in before {
                if right.contains(b) {
                    return false;
                }
            }
        }
    }
    true
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
