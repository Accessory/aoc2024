use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use utils::get_input_path;

#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

impl Equation {
    pub(crate) fn is_valid(&self) -> bool {
        let max_combinations: u64 = 1 << (self.numbers.len() - 1);

        for mut i in 0..max_combinations {
            let mut current = *self.numbers.first().unwrap();
            for n in self.numbers.iter().skip(1) {
                let operation = if (i & 1) == 0 {
                    Operation::Add
                } else {
                    Operation::Multiply
                };
                i >>= 1;
                current = match operation {
                    Operation::Add => current + n,
                    Operation::Multiply => current * n,
                    Operation::Concatenate => panic!("Should not be here!"),
                }
            }
            if current == self.result {
                return true;
            }
        }

        false
    }

    pub(crate) fn is_valid_v2(&self) -> bool {
        let max_combinations: usize = 3_usize.pow((self.numbers.len() - 1) as u32);

        for mut i in 0..max_combinations {
            let mut current = *self.numbers.first().unwrap();
            for n in self.numbers.iter().skip(1) {
                let operation = match i % 3 {
                    0 => Operation::Add,
                    1 => Operation::Multiply,
                    2 => Operation::Concatenate,
                    _ => panic!("Should not be here!"),
                };
                i = i.checked_div(3).unwrap_or(0);
                current = match operation {
                    Operation::Add => current + n,
                    Operation::Multiply => current * n,
                    Operation::Concatenate => concatenate_numbers(current, *n),
                }
            }
            if current == self.result {
                return true;
            }
        }

        false
    }

    // pub(crate) fn count_valid(&self) -> usize {
    //     let mut rtn = 0;
    //     let max_combinations = 1 << (self.numbers.len()-1);
    //
    //     for i in 0..max_combinations {
    //         let mut current = *self.numbers.first().unwrap();
    //         for (p, n) in self.numbers.iter().skip(1).enumerate() {
    //             let operation = if (i & (2_usize.pow(p as u32))) == 0 {
    //                Operation::Add
    //             } else {
    //                 Operation::Multiply
    //             };
    //             current = match operation {
    //                 Operation::Add => {
    //                     current + n
    //                 }
    //                 Operation::Multiply => {
    //                     current * n
    //                 }
    //             }
    //         }
    //         if current == self.result{
    //             rtn += 1;
    //         }
    //     }
    //     rtn
    // }
}

fn concatenate_numbers(mut left: u64, right: u64) -> u64 {
    let mut r = right / 10;
    left *= 10;
    while r > 0 {
        left *= 10;
        r /= 10;
    }
    left + right
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut equations: Vec<Equation> = Vec::new();
    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let split: Vec<&str> = line.split(":").map(|x| x.trim()).collect();
        let result: u64 = split[0].parse().unwrap();
        let numbers: Vec<u64> = split[1]
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        equations.push(Equation { result, numbers });
    }

    // Solve
    // let mut result = 0;
    // let max = equations.len();

    // for (i, equation) in equations.iter().enumerate() {
    // for equation in equations.iter(){
    //     // println!("Equation {i} has {} valid results.", equation.count_valid());
    //     if equation.is_valid() {
    //         result += equation.result;
    //     }
    // }

    let result: u64 = equations
        .into_par_iter()
        .filter_map(|equation| {
            if equation.is_valid() {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum();

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut equations: Vec<Equation> = Vec::new();
    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let split: Vec<&str> = line.split(":").map(|x| x.trim()).collect();
        let result: u64 = split[0].parse().unwrap();
        let numbers: Vec<u64> = split[1]
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        equations.push(Equation { result, numbers });
    }

    // Solve
    // let mut result = 0;
    // // let max = equations.len();
    //
    // // for (i, equation) in equations.iter().enumerate() {
    // for equation in equations.iter(){
    //     // println!("Equation {i} has {} valid results.", equation.count_valid());
    //     // println!("Handling Equation {} of {max}", i + 1);
    //     if equation.is_valid() || equation.is_valid_v2() {
    //         result += equation.result;
    //     }
    // }

    let result: u64 = equations
        .into_par_iter()
        .filter_map(|equation| {
            if equation.is_valid_v2() {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum();

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
