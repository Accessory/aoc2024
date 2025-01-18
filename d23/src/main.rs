use fxhash::{FxHashMap, FxHashSet};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::Path;
use utils::get_input_path;

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split("-");

        let (left, right) = (split.next().unwrap(), split.next().unwrap());

        graph
            .entry(left.to_string())
            .or_default()
            .push(right.to_string());
        graph
            .entry(right.to_string())
            .or_default()
            .push(left.to_string());
    }

    // Solve
    let mut seen = HashSet::new();
    let mut results = Vec::new();
    for (key1, value1) in &graph {
        for key2 in value1 {
            let value2 = graph.get(key2).unwrap();
            for key3 in value2 {
                let hash = create_hash(key1, key2, key3);
                if !seen.insert(hash) {
                    continue;
                }

                let value3 = graph.get(key3).unwrap();
                if value3.contains(key1) {
                    let tmp = vec![key1, key2, key3];
                    // tmp.sort();
                    results.push(tmp);
                }
            }
        }
    }

    // Result
    let mut result = 0;

    // results.sort();
    for r in results {
        // println!("{},{},{}", r[0], r[1], r[2]);
        if r[0].as_bytes()[0] == b't' || r[1].as_bytes()[0] == b't' || r[2].as_bytes()[0] == b't' {
            result += 1;
        }
    }

    println!("Result of part 1 is {}", result);
}

struct SearchContext<'a> {
    position: &'a str,
    check_for_keys: Vec<&'a str>,
}

fn run2(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut graph: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();

    // Parse
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split("-");

        let (left, right) = (split.next().unwrap(), split.next().unwrap());

        graph
            .entry(left.to_string())
            .or_default()
            .insert(right.to_string());
        graph
            .entry(right.to_string())
            .or_default()
            .insert(left.to_string());
    }

    // Solve
    let mut seen = FxHashSet::default();
    let mut best_network = Vec::new();
    for key1 in graph.keys() {
        let mut queue = vec![SearchContext {
            position: key1,
            check_for_keys: vec![],
        }];

        'outer: while let Some(item) = queue.pop() {
            let mut network = item.check_for_keys.clone();
            network.push(item.position);
            network.sort();

            let hash = create_hash2(&network);
            if !seen.insert(hash) {
                continue 'outer;
            }

            let values = graph.get(item.position).unwrap();
            if values.len() < best_network.len() {
                continue 'outer;
            }

            for &check_for_key in &item.check_for_keys {
                if !values.contains(check_for_key) {
                    continue 'outer;
                }
            }

            for value in values {
                queue.push(SearchContext {
                    position: value,
                    check_for_keys: network.clone(),
                })
            }

            if network.len() > best_network.len() {
                best_network = network;
            }
        }
    }

    let result = best_network.join(",");
    println!("Result of part 2 is {}", result);
}

fn main() {
    let input_file = get_input_path(env!("CARGO_MANIFEST_DIR"));

    println!("Running {}", env!("CARGO_PKG_NAME"));
    println!("InputFile: {}", input_file.display());

    run(input_file.as_path());
    run2(input_file.as_path());
}

fn create_hash(key1: &str, key2: &str, key3: &str) -> u64 {
    let mut items = vec![key1, key2, key3];
    items.sort();
    let mut hasher = DefaultHasher::new();
    items.hash(&mut hasher);
    hasher.finish()
}

fn create_hash2(keys: &Vec<&str>) -> u64 {
    let mut hasher = DefaultHasher::new();
    keys.hash(&mut hasher);
    hasher.finish()
}

// fn values_contains_key(values: &Vec<String>, key: &str) -> bool {
//     for value in values {
//         if value == key {
//             return true;
//         }
//     }
//     false
// }

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
