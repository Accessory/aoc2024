#![feature(array_windows)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use utils::get_input_path;
use utils::grid_point::GridPoint;

fn get_num_pad() -> HashMap<u8, GridPoint> {
    HashMap::from([
        (b'1', GridPoint::new(0, 2)),
        (b'2', GridPoint::new(1, 2)),
        (b'3', GridPoint::new(2, 2)),
        (b'4', GridPoint::new(0, 1)),
        (b'5', GridPoint::new(1, 1)),
        (b'6', GridPoint::new(2, 1)),
        (b'7', GridPoint::new(0, 0)),
        (b'8', GridPoint::new(1, 0)),
        (b'9', GridPoint::new(2, 0)),
        (b'0', GridPoint::new(1, 3)),
        (b'A', GridPoint::new(2, 3)),
    ])
}

fn get_key_pad() -> HashMap<u8, GridPoint> {
    HashMap::from([
        (b'^', GridPoint::new(1, 0)),
        (b'A', GridPoint::new(2, 0)),
        (b'<', GridPoint::new(0, 1)),
        (b'v', GridPoint::new(1, 1)),
        (b'>', GridPoint::new(2, 1)),
    ])
}

fn run(input_file: &Path) {
    // Preamble
    let mut codes = Vec::with_capacity(5);
    let num_pad = get_num_pad();
    let key_pad = get_key_pad();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let bytes = line.as_bytes();
        codes.push(vec![bytes[0], bytes[1], bytes[2], bytes[3]]);
    }

    // Solve
    let mut results = Vec::with_capacity(codes.len());

    for code in &codes {
        let mut first_sequence = Vec::new();
        let mut first_from = b'A';
        for to in code {
            // println!("From {} To {}", *from as char, *to as char);
            let mut sequence = create_sequence(first_from, *to, &num_pad);
            first_sequence.append(&mut sequence);
            first_sequence.push(b'A');
            first_from = *to;
        }

        let mut second_from = b'A';
        let mut second_sequence = Vec::new();
        for to in first_sequence {
            // println!("From {} To {}", *from as char, *to as char);
            let mut sequence = create_sequence(second_from, to, &key_pad);
            second_sequence.append(&mut sequence);
            second_sequence.push(b'A');
            second_from = to
        }

        let mut third_from = b'A';
        let mut third_sequence = Vec::new();
        for to in second_sequence {
            // println!("From {} To {}", *from as char, *to as char);
            let mut sequence = create_sequence(third_from, to, &key_pad);
            third_sequence.append(&mut sequence);
            third_sequence.push(b'A');
            third_from = to
        }

        print_sequence(&third_sequence);
        println!();
        println!("Sequence Length {}", third_sequence.len());
        results.push(third_sequence.len());
    }

    // Result
    let mut result = 0;

    for (i, code) in codes.iter().enumerate() {
        let number = (code[0] as char).to_digit(10).unwrap() * 100
            + (code[1] as char).to_digit(10).unwrap() * 10
            + (code[2] as char).to_digit(10).unwrap();

        result += results[i] * number as usize;
    }

    println!("Result of part 1 is {}", result);
}

fn print_sequence(sequence: &Vec<u8>) {
    for c in sequence {
        print!("{}", *c as char);
    }
    // println!()
}

fn create_sequence(from: u8, to: u8, map: &HashMap<u8, GridPoint>) -> Vec<u8> {
    let mut rtn = Vec::new();
    let from_point = map[&from];
    let to_point = map[&to];

    if from == b'A' {
        let direction_indicator = if from_point.y > to_point.y {
            b'^'
        } else {
            b'v'
        };

        for _ in 0..from_point.y.abs_diff(to_point.y) {
            rtn.push(direction_indicator);
        }

        let direction_indicator = if from_point.x > to_point.x {
            b'<'
        } else {
            b'>'
        };

        for _ in 0..from_point.x.abs_diff(to_point.x) {
            rtn.push(direction_indicator);
        }
    } else {
        let direction_indicator = if from_point.x > to_point.x {
            b'<'
        } else {
            b'>'
        };

        for _ in 0..from_point.x.abs_diff(to_point.x) {
            rtn.push(direction_indicator);
        }

        let direction_indicator = if from_point.y > to_point.y {
            b'^'
        } else {
            b'v'
        };

        for _ in 0..from_point.y.abs_diff(to_point.y) {
            rtn.push(direction_indicator);
        }
    }

    rtn
}

fn run2(_input_file: &Path) {}

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
