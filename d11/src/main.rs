use fxhash::FxHashMap;
use std::fs;
use std::ops::AddAssign;
use std::path::Path;
use utils::get_input_path;
use utils::utils::{get_digits_count, split_number};

fn run(input_file: &Path) {
    // Preamble
    const NUMBER_OF_BLINKS: usize = 25;

    // Parse
    let mut stones: Vec<u64> = fs::read_to_string(input_file)
        .unwrap()
        .trim()
        .to_string()
        .split_ascii_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..NUMBER_OF_BLINKS {
        let mut next = Vec::with_capacity(stones.len() * 2);
        for stone in stones {
            let (next_stone, optional_next) = check_stone(stone);
            next.push(next_stone);
            if let Some(other_stone) = optional_next {
                next.push(other_stone);
            }
        }
        stones = next;
    }

    // Result
    let result = stones.len();
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    const NUMBER_OF_BLINKS: usize = 75;

    // Parse
    let stones: Vec<u64> = fs::read_to_string(input_file)
        .unwrap()
        .trim()
        .to_string()
        .split_ascii_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    // Prepare
    let mut context: FxHashMap<u64, u64> = FxHashMap::default();

    for stone in stones {
        context.entry(stone).or_default().add_assign(1);
    }

    // Solve
    for _ in 0..NUMBER_OF_BLINKS {
        let mut next: FxHashMap<u64, u64> = FxHashMap::default();
        for (stone, amount) in context {
            let (next_stone, optional_next) = check_stone(stone);
            next.entry(next_stone).or_default().add_assign(amount);
            if let Some(other_stone) = optional_next {
                next.entry(other_stone).or_default().add_assign(amount);
            }
        }
        context = next;
    }

    // Result
    let result: u64 = context.iter().map(|(_, l)| l).sum();
    println!("Result of part 2 is {}", result);
}

fn check_stone(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }
    let digits = get_digits_count(stone);
    if digits % 2 == 0 {
        let (left, right) = split_number(stone, digits / 2);
        return (left, Some(right));
    }
    (stone * 2024, None)
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
