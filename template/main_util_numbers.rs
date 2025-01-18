use std::path::Path;
use utils::{get_input_path, parse_into_i64_vector, parse_into_usize_vector};

fn run(input_file: &Path) {
    // Preamble
    let mut result = 0;
    // Parse
    let values = parse_into_usize_vector(input_file);

    // Solve
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

