#![feature(get_many_mut)]

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{BitAnd, BitOr, BitXor};
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use utils::get_input_path;

#[derive(Debug, Eq, PartialEq)]
enum GateOperation {
    Or,
    And,
    Xor,
}

impl Display for GateOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            GateOperation::Or => "OR",
            GateOperation::And => "AND",
            GateOperation::Xor => "XOR",
        };
        write!(f, "{to_write}")
    }
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

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.left, self.operation, self.right, self.output
        )
    }
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
            let wire_value = line.ends_with('1');
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

fn run2(input_file: &Path) {
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
            let wire_value = line.ends_with('1');
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
    let outputs: Vec<String> = gates.iter().map(|g| g.output.clone()).collect();

    let mut z_len = 0;

    let mut output_map = HashMap::new();

    for (i, gate) in gates.iter().enumerate() {
        if gate.output.as_bytes()[0] == b'z' {
            z_len += 1;
        }
        output_map.insert(gate.output.clone(), i);
    }

    let swapped_outputs = find_z_anomalies(&output_map, &mut gates, z_len);

    set_x_to_gates(&mut value_map, 28872341726885);
    set_y_to_gates(&mut value_map, 28414614475596);

    let x = calculate_x(&value_map);
    let y = calculate_y(&value_map);

    let should_be = x + y;

    // 'outer: for i1 in 0..outputs.len() {
    //     for i2 in i1 + 1..outputs.len() {
    //         let o1 = &outputs[i1];
    //         let o2 = &outputs[i2];
    //
    //         let mut overrides = HashMap::with_capacity(2);
    //         overrides.insert(o1, o2);
    //         overrides.insert(o2, o1);
    //
    //         let result = solve_gates(&value_map, &gates, overrides);
    //
    //         if result == should_be {
    //             swapped_outputs.push((o1.to_string(), o2.to_string()));
    //             break 'outer;
    //         }
    //     }
    // }

    let r1 = AtomicUsize::default();
    let r2 = AtomicUsize::default();

    (0..outputs.len()).into_par_iter().any(|i1| {
        (i1 + 1..outputs.len()).any(|i2| {
            let o1 = &outputs[i1];
            let o2 = &outputs[i2];

            let mut overrides = HashMap::with_capacity(2);
            overrides.insert(o1, o2);
            overrides.insert(o2, o1);

            let result = solve_gates(&value_map, &gates, overrides);

            if result == should_be {
                r1.store(i1, Relaxed);
                r2.store(i2, Relaxed);
                return true;
            }
            false
        })
    });

    // Result
    let mut all_wires = Vec::with_capacity(swapped_outputs.len() * 2 + 2);
    for (l, r) in swapped_outputs {
        all_wires.push(l);
        all_wires.push(r);
    }

    let o1 = &outputs[r1.load(Relaxed)];
    let o2 = &outputs[r2.load(Relaxed)];
    all_wires.push(o1.to_string());
    all_wires.push(o2.to_string());
    all_wires.sort();

    println!("Result of part 2 is {}", all_wires.join(","));
}

fn set_x_to_gates(value_map: &mut HashMap<String, bool>, mut number: u64) {
    for i in 0..64 {
        let key = format!("x{i:<02}");
        if let Some(i) = value_map.get_mut(&key) {
            *i = number % 2 == 1;
        }
        number /= 2;
    }
}

fn set_y_to_gates(value_map: &mut HashMap<String, bool>, mut number: u64) {
    for i in 0..64 {
        let key = format!("y{i:<02}");
        if let Some(i) = value_map.get_mut(&key) {
            *i = number % 2 == 1;
        }
        number /= 2;
    }
}

fn solve_gates(
    init_value_map: &HashMap<String, bool>,
    gates: &[Gate],
    overrides: HashMap<&String, &String>,
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
                    .get(&gate.output)
                    .map_or(gate.output.as_str(), |v| v);
                value_map.insert(output.to_string(), r);
                solved_gates.insert(i);
            }
        }
    }

    calculate_result(value_map)
}

fn find_z_anomalies(
    output_map: &HashMap<String, usize>,
    gates: &mut [Gate],
    z_len: usize,
) -> Vec<(String, String)> {
    let mut rtn = Vec::new();
    for z in 0..z_len - 1 {
        let key = format!("z{z:<02}");
        let g_id = *output_map.get(&key).unwrap();
        let gate = gates.get(g_id).unwrap();
        if gate.operation == GateOperation::Xor {
            continue;
        }
        // println!("Bad gate: {}", gate);
        let expected_nbr_dependents = z * 6;
        let (s, swap_gate) = find_swap_gate(expected_nbr_dependents, output_map, gates);
        // println!("Swap with {swap_gate}");
        rtn.push((gate.output.clone(), swap_gate.output.clone()));
        swap_gates(gates, g_id, s);
    }
    rtn
}

fn swap_gates(gates: &mut [Gate], start: usize, end: usize) {
    let [g1, g2] = gates.get_many_mut([start, end]).unwrap();
    let tmp = g1.output.clone();
    g1.output = g2.output.clone();
    g2.output = tmp;
}

fn find_swap_gate<'a>(
    expected_nbr_dependents: usize,
    output_map: &'a HashMap<String, usize>,
    gates: &'a [Gate],
) -> (usize, &'a Gate) {
    for (g, gate) in gates.iter().enumerate() {
        if gate.operation == GateOperation::Xor {
            let dependents_number = dependents_of_wire(&gate.output, output_map, gates);
            // println!("{dependents_number} == {expected_nbr_dependents}");
            if dependents_number == expected_nbr_dependents {
                return (g, gate);
            }
        }
    }
    panic!()
}

fn dependents_of_wire(
    output: &String,
    output_map: &HashMap<String, usize>,
    gates: &[Gate],
) -> usize {
    let mut rtn = HashSet::new();

    let mut queue = vec![output];

    while let Some(item) = queue.pop() {
        if item.chars().next().is_some_and(|c| c == 'y' || c == 'x') {
            continue;
        }

        let gate = gates.get(*output_map.get(item).unwrap()).unwrap();
        if rtn.insert(gate.left.clone()) {
            queue.push(&gate.left);
        }
        if rtn.insert(gate.right.clone()) {
            queue.push(&gate.right);
        }
    }

    rtn.len()
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
    use utils::get_test_input_path;

    use crate::run;

    #[test]
    fn test_input_part_1() {
        run(&get_test_input_path(env!("CARGO_MANIFEST_DIR")));
    }

    // #[test]
    // fn test_input_part_2() {
    //     run2(&get_test_input_2_path(env!("CARGO_MANIFEST_DIR")));
    // }
}
