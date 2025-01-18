use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{AddAssign, SubAssign};
use std::path::Path;

use utils::get_input_path;

enum ParserState {
    FirstLine,
    Key,
    Lock,
}

#[allow(clippy::needless_range_loop)]
fn run(input_file: &Path) {
    // Preamble
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let file = File::open(input_file).unwrap();

    let mut current_row_counter: [u8; 5] = [0, 0, 0, 0, 0];
    let mut parser_state = ParserState::FirstLine;

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() {
            match parser_state {
                ParserState::Key => {
                    keys.push(current_row_counter);
                }
                ParserState::Lock => {
                    locks.push(current_row_counter);
                }
                ParserState::FirstLine => panic!("Should not be here"),
            }
            parser_state = ParserState::FirstLine;
            continue;
        }
        match parser_state {
            ParserState::FirstLine => {
                if line.as_bytes()[0] == b'#' {
                    parser_state = ParserState::Lock;
                    current_row_counter = [0, 0, 0, 0, 0]
                } else {
                    parser_state = ParserState::Key;
                    current_row_counter = [5, 5, 5, 5, 5]
                }
            }
            ParserState::Key => {
                for i in 0..5 {
                    if line.as_bytes()[i] == b'.' {
                        current_row_counter[i].sub_assign(1);
                    }
                }
            }
            ParserState::Lock => {
                for i in 0..5 {
                    if line.as_bytes()[i] == b'#' {
                        current_row_counter[i].add_assign(1);
                    }
                }
            }
        }
    }

    match parser_state {
        ParserState::Key => {
            keys.push(current_row_counter);
        }
        ParserState::Lock => {
            locks.push(current_row_counter);
        }
        ParserState::FirstLine => panic!("Should not be here"),
    }

    // Solve
    let mut result: usize = 0;
    for lock in &locks {
        'keys: for key in &keys {
            for i in 0..5 {
                if (lock[i] + key[i]) > 5 {
                    continue 'keys;
                }
            }
            result += 1;
        }
    }

    // Result
    println!("Result of part 1 is {}", result);
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
