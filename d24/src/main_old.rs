use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::ops::{BitAnd, BitOr, BitXor};
use std::path::Path;
use utils::get_input_path;

enum GateOperation {
    Or,
    And,
    Xor,
}

impl From<&str> for GateOperation {
    fn from(value: &str) -> Self {
        match value {
            "XOR" => GateOperation::Xor,
            "OR" => GateOperation::Or,
            "AND" => GateOperation::And,
            _ => panic!("Should not be here!"),
        }
    }
}

struct Gate {
    left: String,
    right: String,
    output: String,
    operation: GateOperation,
}

fn run(input_file: &Path) {
    // Preamble
    let mut value_map = HashMap::new();
    let mut gates = Vec::new();

    let file = File::open(input_file).unwrap();

    // Parse
    let reader = BufReader::new(file);

    let mut is_init_values = true;
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.is_empty() {
            is_init_values = false;
            continue;
        }

        if is_init_values {
            let wire = line[0..3].to_string();
            let wire_value = line.chars().last().unwrap() == '1';
            value_map.insert(wire, wire_value);
        } else {
            let mut split = line.split_ascii_whitespace();
            let left = split.next().unwrap().to_string();
            let operation = split.next().unwrap().into();
            let right = split.next().unwrap().to_string();
            let output = split.last().unwrap().to_string();
            gates.push(Gate {
                left,
                right,
                output,
                operation,
            })
        }
    }

    // Solve
    let mut solved_gates = HashSet::with_capacity(gates.len());
    loop {
        if solved_gates.len() == gates.len() {
            break;
        }

        for (i, gate) in gates.iter().enumerate() {
            if solved_gates.contains(&i) {
                continue;
            }

            let result = solve_gate(gate, &value_map);

            if let Some(r) = result {
                value_map.insert(gate.output.clone(), r);
                solved_gates.insert(i);
            }
        }
    }

    let result = calculate_result(value_map);
    // Result
    println!("Result of part 1 is {}", result);
}

#[cfg(test)]
const SWITCHES: usize = 4;

#[cfg(not(test))]
const SWITCHES: usize = 8;

fn run2(input_file: &Path) {
    // Preamble
    let mut value_map = HashMap::new();
    let mut gates = Vec::new();

    let file = File::open(input_file).unwrap();

    // Parse
    let reader = BufReader::new(file);

    let mut is_init_values = true;
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        if line.is_empty() {
            is_init_values = false;
            continue;
        }

        if is_init_values {
            let wire = line[0..3].to_string();
            let wire_value = line.chars().last().unwrap() == '1';
            value_map.insert(wire, wire_value);
        } else {
            let mut split = line.split_ascii_whitespace();
            let left = split.next().unwrap().to_string();
            let operation = split.next().unwrap().into();
            let right = split.next().unwrap().to_string();
            let output = split.last().unwrap().to_string();
            gates.push(Gate {
                left,
                right,
                output,
                operation,
            })
        }
    }

    // Solve
    let outputs: Vec<String> = gates.iter().map(|g| g.output.clone()).collect();

    let x = calculate_x(&value_map);
    let y = calculate_y(&value_map);

    #[cfg(test)]
    let should_be = x.bitand(y);

    #[cfg(not(test))]
    let should_be = x + y;

    let mut final_overrides: Option<Vec<String>> = None;

    let mut seen = HashSet::new();

    for perm in outputs.iter().permutations(outputs.len()) {
        let (own_hash, reversed_hash) = create_hash_key(&perm);

        // let mut key: Vec<String> = perm[0..SWITCHES].iter().map(|v| v.to_string()).collect();
        // key.sort();
        // println!("Current Key: {}", key.join(","));

        // if key.join(",") == "z00,z01,z02,z05" {
        //     println!("Stop");
        // }

        if seen.contains(&own_hash) || !seen.insert(reversed_hash) {
            continue;
        }

        let overrides = create_overrides(&perm);

        let result = solve_gates(&value_map, &gates, overrides);

        if result == should_be {
            final_overrides = Some(perm[0..SWITCHES].iter().map(|v| v.to_string()).collect());
            break;
        }
    }

    let result = 0;
    // dbg!(final_overrides);
    // Result
    println!("Result of part 2 is {}", final_overrides.unwrap().join(","));
}

fn create_overrides<'a>(perm: &'a Vec<&'a String>) -> HashMap<&'a str, &'a str> {
    let mut rtn = HashMap::with_capacity(SWITCHES);

    for i in 0..SWITCHES / 2 {
        let o1 = i * 2;
        let o2 = o1 + 1;
        rtn.insert(perm[o1].as_str(), perm[o2].as_str());
        rtn.insert(perm[o2].as_str(), perm[o1].as_str());
    }

    rtn
}

fn create_hash_key(perm: &Vec<&String>) -> (u64, u64) {
    let mut items = Vec::from_iter(&perm[0..SWITCHES]);
    let mut hasher = DefaultHasher::default();
    items.hash(&mut hasher);

    for i in 0..SWITCHES / 2 {
        let o1 = i * 2;
        let o2 = o1 + 1;
        items.swap(o1,o2);
    }

    let mut hasher2 = DefaultHasher::default();
    items.hash(&mut hasher2);
    (hasher.finish(), hasher2.finish())
}

fn solve_gates(
    init_value_map: &HashMap<String, bool>,
    gates: &Vec<Gate>,
    overrides: HashMap<&str, &str>,
) -> u64 {
    let mut value_map = init_value_map.clone();
    let mut solved_gates = HashSet::with_capacity(gates.len());
    let mut last_solved_gates_amount = usize::MAX;
    loop {
        if solved_gates.len() == gates.len() {
            break;
        }

        if solved_gates.len() == last_solved_gates_amount {
            break;
        }

        last_solved_gates_amount = solved_gates.len();

        for (i, gate) in gates.iter().enumerate() {
            if solved_gates.contains(&i) {
                continue;
            }

            let result = solve_gate(gate, &value_map);

            if let Some(r) = result {
                let output = overrides
                    .get(gate.output.as_str())
                    .map_or(gate.output.as_str(), |v| v);
                value_map.insert(output.to_string(), r);
                solved_gates.insert(i);
            }
        }
    }

    calculate_result(value_map)
}

fn solve_gate(gate: &Gate, value_map: &HashMap<String, bool>) -> Option<bool> {
    let left = value_map.get(&gate.left)?;
    let right = value_map.get(&gate.right)?;

    let rtn = match gate.operation {
        GateOperation::Or => left.bitor(right),
        GateOperation::And => left.bitand(right),
        GateOperation::Xor => left.bitxor(right),
    };

    Some(rtn)
}

fn calculate_y(value_map: &HashMap<String, bool>) -> u64 {
    let mut rtn = 0;
    for i in (0..64).rev() {
        let key = format!("y{i:<02}");
        rtn <<= 1;
        if *value_map.get(&key).unwrap_or(&false) {
            rtn += 1;
        }
    }
    rtn
}

fn calculate_x(value_map: &HashMap<String, bool>) -> u64 {
    let mut rtn = 0;
    for i in (0..64).rev() {
        let key = format!("x{i:<02}");
        rtn <<= 1;
        if *value_map.get(&key).unwrap_or(&false) {
            rtn += 1;
        }
    }
    rtn
}

fn calculate_result(value_map: HashMap<String, bool>) -> u64 {
    let mut rtn = 0;
    for i in (0..64).rev() {
        let key = format!("z{i:<02}");
        rtn <<= 1;
        if *value_map.get(&key).unwrap_or(&false) {
            rtn += 1;
        }
    }
    rtn
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
