use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use utils::{get_input_path};

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let _line = line.unwrap().trim().to_string();
    }

    // Solve
    let result = 0;
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
