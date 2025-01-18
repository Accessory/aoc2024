use fxhash::FxHashMap;
use rayon::prelude::*;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, BitXor};
use std::path::Path;
use utils::{get_input_path, parse_file_into};

#[derive(Debug, Copy, Clone)]
struct SecretNumber(usize);

impl From<String> for SecretNumber {
    fn from(value: String) -> Self {
        Self(value.parse().unwrap())
    }
}

impl Display for SecretNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SecretNumber {
    pub(crate) fn next_bananas_and_div(&mut self) -> (i8, i8) {
        let bananas = self.bananas();
        self.next();
        let next_bananas = self.bananas();
        (next_bananas, next_bananas - bananas)
    }

    pub(crate) fn bananas(&self) -> i8 {
        (self.0 % 10) as i8
    }

    fn next(&mut self) {
        let s1 = self.0 * 64;
        let s1m = self.0.bitxor(s1);
        let s1mo = s1m % 16777216;
        let s2 = s1mo / 32;
        let s2m = s1mo.bitxor(s2);
        let s2mo = s2m % 16777216;
        let s3 = s2mo * 2048;
        let s3m = s2mo.bitxor(s3);
        let s3mo = s3m % 16777216;
        self.0 = s3mo
    }
}

fn run(input_file: &Path) {
    // Preamble
    // Parse
    let mut values: Vec<SecretNumber> = parse_file_into(input_file);

    // Solve
    let result: usize = values
        .par_iter_mut()
        .map(|value| {
            for _ in 0..2000 {
                value.next();
            }
            value.0
        })
        .sum();
    // Result
    println!("Result of part 1 is {}", result);
}

struct RollingContainer {
    position: usize,
    data: [i8; 4],
}

impl RollingContainer {
    fn insert(&mut self, number: i8) {
        self.data[self.position] = number;
        if self.position == 3 {
            self.position = 0
        } else {
            self.position += 1;
        }
    }

    fn get_data(&self) -> [i8; 4] {
        let mut rtn = [i8::MAX, i8::MAX, i8::MAX, i8::MAX];
        for (i, item) in rtn.iter_mut().enumerate() {
            let pos = (self.position + i) % 4;
            *item = self.data[pos];
        }
        rtn
    }
}

fn run2(input_file: &Path) {
    // Parse
    let mut values: Vec<SecretNumber> = parse_file_into(input_file);

    // Solve
    let outer_results = values
        .par_iter_mut()
        .map(|value| {
            let mut last_four_div = RollingContainer {
                position: 0,
                data: [i8::MAX, i8::MAX, i8::MAX, i8::MAX],
            };

            let mut last_four_bananas = RollingContainer {
                position: 0,
                data: [i8::MAX, i8::MAX, i8::MAX, i8::MAX],
            };

            let mut inner_results: FxHashMap<[i8; 4], usize> = FxHashMap::default();
            last_four_div.insert(0);
            last_four_bananas.insert(value.bananas());
            for i in 0..2000 {
                let (bananas, div) = value.next_bananas_and_div();
                last_four_div.insert(div);
                last_four_bananas.insert(bananas);

                if i > 3 {
                    let key = last_four_div.get_data();
                    inner_results.entry(key).or_insert(bananas as usize);
                }
            }
            inner_results
        })
        .reduce(FxHashMap::default, |mut outer_results, inner_results| {
            inner_results.iter().for_each(|(key, &value)| {
                outer_results.entry(*key).or_default().add_assign(value);
            });
            outer_results
        });

    // Result
    let (sequence, bananas) = outer_results
        .iter()
        .max_by_key(|(_key, &value)| value)
        .unwrap();

    println!("Best sequence {:?} with {} bananas", sequence, bananas);
    println!("Result of part 2 is {}", bananas);
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
