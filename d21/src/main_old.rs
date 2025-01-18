#![feature(array_windows)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use utils::get_input_path;
use utils::grid_direction::GridDirection;
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

fn get_arrow_pad() -> HashMap<u8, GridPoint> {
    HashMap::from([
        (b'^', GridPoint::new(1, 0)),
        (b'A', GridPoint::new(2, 0)),
        (b'<', GridPoint::new(0, 1)),
        (b'v', GridPoint::new(1, 1)),
        (b'>', GridPoint::new(2, 1)),
    ])
}

struct InputContext {
    layer: u8,
    pos: usize,
    origin: Vec<usize>,
    sequence: Vec<u8>,
}

impl InputContext {
    fn new(sequence: Vec<u8>) -> Self {
        Self {
            layer: 0,
            pos: 0,
            origin: Vec::new(),
            sequence,
        }
    }
}

fn run(input_file: &Path) {
    // Preamble
    let mut queue = Vec::with_capacity(5);
    let num_pad = get_num_pad();
    let arrow_pad = get_arrow_pad();

    let valid_num_pad: HashSet<GridPoint> = num_pad.iter().map(|x| *x.1).collect();
    let valid_arrow_pad: HashSet<GridPoint> = arrow_pad.iter().map(|x| *x.1).collect();

    let mut memo: HashMap<(u8, u8), Vec<Vec<u8>>> = HashMap::new();
    let mut memo2: HashMap<(u8, u8), Vec<Vec<u8>>> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let bytes = line.as_bytes();
        queue.push(InputContext::new(vec![
            bytes[0], bytes[1], bytes[2], bytes[3],
        ]));
        break;
    }

    // Solve
    let results: Vec<usize> = Vec::with_capacity(queue.len());

    let mut layer1 = BTreeMap::new();
    let mut layer2 = BTreeMap::new();
    let mut layer3 = BTreeMap::new();

    let mut debug:BTreeMap<u8, HashSet<usize>> = BTreeMap::new();

    while let Some(item) = queue.pop() {
        let from = if item.pos == 0 {
            b'A'
        } else {
            item.sequence[item.pos - 1]
        };
        let to = if item.sequence.len() == item.pos {
            b'A'
        } else {
            item.sequence[item.pos]
        };

        debug.entry(item.layer).or_default().insert(item.sequence.len());

        match item.layer {
            0 => {
                let sequences = create_sequences(from, to, &num_pad, &valid_num_pad, &mut memo);
                for sequence in sequences {
                    queue.push(InputContext {
                        layer: 1,
                        pos: 0,
                        origin: vec![item.pos],
                        sequence,
                    })
                }
            }
            1 => {
                // println!("Position {} {}", item.origin.first().unwrap(), item.origin.last().unwrap());
                // print_sequence(&item.sequence);
                let f = *item.origin.first().unwrap();
                layer1.entry(f).or_insert(item.sequence.clone());
                // println!();
                let sequences =
                    create_sequences(from, to, &arrow_pad, &valid_arrow_pad, &mut memo2);
                for sequence in sequences {
                    // println!("New sequence: {:?}", sequence);
                    let mut origin = item.origin.clone();
                    origin.push(item.pos);
                    queue.push(InputContext {
                        layer: 2,
                        pos: 0,
                        origin,
                        sequence,
                    })
                }
            }
            2 => {
                // println!("Position {} {}", item.origin.first().unwrap(), item.origin.last().unwrap());
                // print_sequence(&item.sequence);
                // println!();
                let f = *item.origin.first().unwrap();
                let f2 = *item.origin.last().unwrap();
                // layer2.entry((f, f2)).or_insert(item.sequence.clone());
                layer2.insert((f,f2), item.sequence.clone());
                let sequences =
                    create_sequences(from, to, &arrow_pad, &valid_arrow_pad, &mut memo2);

                for sequence in sequences {
                    // println!("New sequence: {:?}", sequence);
                    let mut origin = item.origin.clone();
                    origin.push(item.pos);
                    queue.push(InputContext {
                        layer: 3,
                        pos: 0,
                        origin,
                        sequence,
                    })
                }
            }
            3 => {
                let f = *item.origin.first().unwrap();
                let f2 = item.origin[1];
                let f3 = *item.origin.last().unwrap();

                layer3.insert((f, f2, f3), item.sequence.clone());

                // let entry = layer3.entry((f, f2, f3)).or_insert(item.sequence.clone());
                // if entry.len() >= item.sequence.len() {
                //     *entry = item.sequence.clone()
                // }
            }
            _ => panic!("Should not be here!"),
        }

        let next_pos = item.pos + 1;
        if item.pos < item.sequence.len() {
            queue.push(InputContext {
                layer: item.layer,
                pos: next_pos,
                origin: item.origin,
                sequence: item.sequence,
            });
        }
    }

    // for i in 0..4 {
    //     print!("{}A", seq_to_string(layer1.get(&i).unwrap()))
    // }
    // println!();

    let rtn = layer2
        .iter()
        .map(|(_, v)| seq_to_string(v))
        .collect::<Vec<String>>()
        .join("A");
    println!("{rtn}");

    for d in debug {
        dbg!(d);
    }

    // Result
    let result = rtn.len();

    // for (i, code) in queue.iter().enumerate() {
    //     let number = (code[0] as char).to_digit(10).unwrap() * 100
    //         + (code[1] as char).to_digit(10).unwrap() * 10
    //         + (code[2] as char).to_digit(10).unwrap();
    //
    //     result += results[i] * number as usize;
    //     break;
    // }

    println!("Result of part 1 is {}", result);
}

fn seq_to_string(sequence: &Vec<u8>) -> String {
    let mut rtn = String::with_capacity(sequence.len());

    for s in sequence {
        rtn.push(*s as char);
    }

    rtn
}

fn print_sequence(sequence: &Vec<u8>) {
    for c in sequence {
        print!("{}", *c as char);
    }
    // println!()
}

struct SequenceContext {
    position: GridPoint,
    directions: Vec<u8>,
}

fn create_sequences(
    from: u8,
    to: u8,
    map: &HashMap<u8, GridPoint>,
    valid: &HashSet<GridPoint>,
    memos: &mut HashMap<(u8, u8), Vec<Vec<u8>>>,
) -> Vec<Vec<u8>> {
    let key = (from, to);
    if let Some(saved) = memos.get(&key) {
        return saved.clone();
    }

    let mut rtn = Vec::new();
    let start = map[&from];
    let goal = map[&to];

    let mut queue = vec![SequenceContext {
        position: start,
        directions: Vec::new(),
    }];

    let max = start.manhatten_distance(&goal);

    while let Some(context) = queue.pop() {
        if context.directions.len() > max {
            continue;
        }

        if context.position == goal {
            rtn.push(context.directions);
            continue;
        }

        for direction in GridDirection::get_directions() {
            if let Some(next) = context
                .position
                .next_by_direction_with_check(&direction, 3, 3)
            {
                if valid.contains(&next) {
                    let mut next_directions = context.directions.clone();
                    next_directions.push(direction.to_char_u8());
                    queue.push(SequenceContext {
                        position: next,
                        directions: next_directions,
                    });
                }
            }
        }
    }

    memos.insert((from, to), rtn.clone());
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
