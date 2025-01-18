use std::collections::HashMap;
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

        // let n = self.0;
        // let a = (n ^ (n << 6)) & 0xFFFFFF;
        // let b = a ^ (a >> 5);
        // let r = (b ^ (b << 11)) & 0xFFFFFF;
        // self.0 = r;
    }
}

fn run(input_file: &Path) {
    // Preamble
    let mut result = 0;
    // Parse
    let values: Vec<SecretNumber> = parse_file_into(input_file);

    // Solve
    for mut value in values {
        // println!("Start number: {}", &value);
        for _ in 0..2000 {
            value.next();
        }
        result += value.0;
        // println!("After 2000 iterations {}", &value);
    }
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
        for i in 0..4 {
            let pos = (self.position + i) % 4;
            rtn[i] = self.data[pos];
        }
        rtn
    }
}

fn run2(input_file: &Path) {
    // Preamble
    // Parse
    let values: Vec<SecretNumber> = parse_file_into(input_file);

    let mut outer_results: HashMap<[i8; 4], usize> = HashMap::new();
    // let mut inner_result_list = Vec::with_capacity(values.len());

    // Solve
    for mut value in values {

        let mut last_four_div = RollingContainer {
            position: 0,
            data: [i8::MAX, i8::MAX, i8::MAX, i8::MAX],
        };

        let mut last_four_bananas = RollingContainer {
            position: 0,
            data: [i8::MAX, i8::MAX, i8::MAX, i8::MAX],
        };


        let mut inner_results: HashMap<[i8; 4], i8> = HashMap::new();
        // println!("Start number: {}", &value);
        last_four_div.insert(0);
        last_four_bananas.insert(value.bananas());
        for i in 0..2000 {
            // if [0, 0, 0, 1] == last_four_div.get_data() {
            //     println!("After {i} in {value}");
            //     println!("Bananas: {:?}", last_four_bananas.get_data());
            //     println!("Div    : {:?}", last_four_div.get_data());
            // }
            let (bananas, div) = value.next_bananas_and_div();
            last_four_div.insert(div);
            last_four_bananas.insert(bananas);

            if i > 3 {
                // let entry = inner_results
                //     .entry(last_four_div.get_data())
                //     .or_insert(bananas as usize);
                // if *entry < bananas as usize {
                //     *entry = bananas as usize;
                // }
                let key = last_four_div.get_data();
                if !inner_results.contains_key(&key) {
                    inner_results.insert(key, bananas);
                }
            }
        }
        inner_results.iter().for_each(|(key, &value)| {
            outer_results.entry(*key).or_default().add_assign(value as usize);
        });
        // inner_result_list.push(inner_results);
    }
    // Result
    // let mut tmp: Vec<(&[i8; 4], usize)> = outer_results
    //     .iter()
    //     .map(|(key, &value)| (key, value))
    //     .collect();
    //
    // tmp.sort_by(|v, v2| v2.1.cmp(&v.1));
    //
    // for i in 0..10 {
    //     println!("{:?}", tmp[i]);
    // }

    let (_sequence, bananas) = outer_results
        .iter()
        .max_by_key(|(_key, &value)| value)
        .unwrap();
    // println!("Best sequence {:?} with {}", sequence, bananas);
    println!("Result of part 2 is {}", bananas);

    // let mut best = 0;
    // for a in -9..10 {
    //     for b in -9..10 {
    //         for c in -9..10 {
    //             for d in -9..10 {
    //                 let mut current = 0;
    //                 for results in &inner_result_list{
    //                     if let Some(&value ) = results.get(&[a,b,c,d]) {
    //                         current += value;
    //                     }
    //                 }
    //                 best = best.max(current);
    //             }
    //         }
    //     }
    // }
    // println!("Other result for 2 {best}");
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
