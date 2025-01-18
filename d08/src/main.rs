#![feature(array_windows)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use utils::get_input_path;
use utils::point::MapPoint;

fn run(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut data: HashMap<char, Vec<MapPoint>> = HashMap::new();
    // Parse
    let reader = BufReader::new(file);

    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        max_x = max_x.max(line.len());
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                data.entry(c)
                    .or_default()
                    .push(MapPoint::new(x as i64, y as i64));
            }
        }
        max_y = y;
    }
    max_y += 1;

    let mut antinodes = HashSet::new();

    // Solve
    for de in data.iter() {
        for i1 in 0..de.1.len() {
            for i2 in i1 + 1..de.1.len() {
                let e1 = &de.1[i1];
                let e2 = &de.1[i2];
                let (x, y) = e2.diff(e1);
                let (hx, hy) = e1.add(x, y);
                // println!("Antinode: {} - {}", hx, hy);
                if hx >= 0 && hx < (max_x as i64) && hy >= 0 && hy < (max_y as i64) {
                    let _ = antinodes.insert((hx, hy));
                }
                let (x, y) = e1.diff(e2);
                let (hx, hy) = e2.add(x, y);
                // println!("Antinode: {} - {}", hx, hy);
                if hx >= 0 && hx < (max_x as i64) && hy >= 0 && hy < (max_y as i64) {
                    let _ = antinodes.insert((hx, hy));
                }
            }
        }
    }

    // Result
    // for y in 0..max_y as i64 {
    //     for x in 0..max_x as i64 {
    //         if let Some((c, _)) = data
    //             .iter()
    //             .find(|(_, mp)| mp.iter().find(|x1| x1.y == y && x1.x == x).is_some())
    //         {
    //             print!("{c}");
    //             continue;
    //         }
    //         if antinodes.contains(&(x, y)) {
    //             print!("#");
    //             continue;
    //         }
    //         print!(".");
    //     }
    //     println!();
    // }

    let result = antinodes.len();
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    let file = File::open(input_file).unwrap();
    let mut data: HashMap<char, Vec<MapPoint>> = HashMap::new();

    // Parse
    let reader = BufReader::new(file);

    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        max_x = max_x.max(line.len());
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                data.entry(c)
                    .or_default()
                    .push(MapPoint::new(x as i64, y as i64));
            }
        }
        max_y = y;
    }
    max_y += 1;

    let mut antinodes = HashSet::new();

    // Solve
    for de in data.iter() {
        for i1 in 0..de.1.len() {
            for i2 in i1 + 1..de.1.len() {
                let e1 = &de.1[i1];
                let e2 = &de.1[i2];
                let (x, y) = e2.diff(e1);
                let mut current_x = e1.x;
                let mut current_y = e1.y;
                let _ = antinodes.insert((current_x, current_y));
                loop {
                    current_x -= x;
                    current_y -= y;
                    if current_x < 0
                        || current_x >= max_x as i64
                        || current_y < 0
                        || current_y >= max_y as i64
                    {
                        break;
                    }
                    // println!("Antinode: {} - {}", current_x, current_y);
                    let _ = antinodes.insert((current_x, current_y));
                }
                let (x, y) = e1.diff(e2);
                let mut current_x = e2.x;
                let mut current_y = e2.y;
                let _ = antinodes.insert((current_x, current_y));
                loop {
                    current_x -= x;
                    current_y -= y;
                    if current_x < 0
                        || current_x >= max_x as i64
                        || current_y < 0
                        || current_y >= max_y as i64
                    {
                        break;
                    }
                    // println!("Antinode: {} - {}", current_x, current_y);
                    let _ = antinodes.insert((current_x, current_y));
                }
            }
        }
    }

    // Result
    // for y in 0..max_y as i64 {
    //     for x in 0..max_x as i64 {
    //         if antinodes.contains(&(x, y)) {
    //             print!("#");
    //             continue;
    //         }
    //         if let Some((c, _)) = data
    //             .iter()
    //             .find(|(_, mp)| mp.iter().find(|x1| x1.y == y && x1.x == x).is_some())
    //         {
    //             print!("{c}");
    //             continue;
    //         }
    //         print!(".");
    //     }
    //     println!();
    // }

    let result = antinodes.len();
    println!("Result of part 1 is {}", result);
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
    use crate::run;
    use crate::run2;
    use utils::get_test_input_path;

    #[test]
    fn test_input_part_1() {
        run(&get_test_input_path(env!("CARGO_MANIFEST_DIR")));
    }

    #[test]
    fn test_input_part_2() {
        run2(&get_test_input_path(env!("CARGO_MANIFEST_DIR")));
    }
}
