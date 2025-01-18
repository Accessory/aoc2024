use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use utils::get_input_path;

#[cfg(test)]
const SPACE_X: i64 = 11;
#[cfg(test)]
const SPACE_Y: i64 = 7;

#[cfg(not(test))]
const SPACE_X: i64 = 101;
#[cfg(not(test))]
const SPACE_Y: i64 = 103;

const MIDDLE_X: i64 = SPACE_X / 2;
const MIDDLE_Y: i64 = SPACE_Y / 2;

#[derive(Debug, Copy, Clone)]
struct Robot {
    position_x: i64,
    position_y: i64,
    velocity_x: i64,
    velocity_y: i64,
}

impl Robot {
    fn is_on(&self, x: i64, y: i64) -> bool {
        self.position_x == x && self.position_y == y
    }
    fn tick(&mut self) {
        self.position_x += SPACE_X + self.velocity_x;
        self.position_y += SPACE_Y + self.velocity_y;
        self.position_x %= SPACE_X;
        self.position_y %= SPACE_Y;
        // if self.position_x < 0 {
        //     self.position_x += SPACE_X;
        // }
        // if self.position_y < 0 {
        //     self.position_y += SPACE_Y;
        // }
    }
}

impl From<String> for Robot {
    fn from(value: String) -> Self {
        let mut split = value.split_ascii_whitespace();
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        let mut left_number_split = left[2..].split(",");
        let position_x = left_number_split.next().unwrap().parse().unwrap();
        let position_y = left_number_split.next().unwrap().parse().unwrap();

        let mut right_number_split = right[2..].split(",");
        let velocity_x = right_number_split.next().unwrap().parse().unwrap();
        let velocity_y = right_number_split.next().unwrap().parse().unwrap();

        Self {
            position_x,
            position_y,
            velocity_y,
            velocity_x,
        }
    }
}

fn run(input_file: &Path) {
    // Preamble
    // println!("Space {SPACE_X}:{SPACE_Y} - Middle {MIDDLE_X}:{MIDDLE_Y}");
    const SECONDS_TO_SIMULATE: usize = 100;

    let file = File::open(input_file).unwrap();

    let mut robots: Vec<Robot> = Vec::new();

    // Parse
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        robots.push(line.into())
    }

    for _i in 0..SECONDS_TO_SIMULATE {
        for robot in robots.iter_mut() {
            robot.tick();
        }
        // println!("Print Grid of Second: {_i}");
    }
    // print_grid(&robots);

    // Solve
    let (top_right, top_left, down_right, down_left) = robots_in_quadrant(&robots);
    // Result
    // println!("TL: {top_left}, TR: {top_right}, DL: {down_left}, DR: {down_right}");
    let result = top_right * top_left * down_right * down_left;
    println!("Result of part 1 is {}", result);
}

fn robots_in_quadrant(robots: &Vec<Robot>) -> (usize, usize, usize, usize) {
    let mut top_left = 0;
    let mut top_right = 0;
    let mut down_left = 0;
    let mut down_right = 0;

    for robot in robots {
        let x = robot.position_x;
        let y = robot.position_y;
        if y == MIDDLE_Y || x == MIDDLE_X {
            continue;
        }

        let top = y < MIDDLE_Y;
        let left = x < MIDDLE_X;

        match (top, left) {
            (true, true) => top_left += 1,
            (true, false) => top_right += 1,
            (false, true) => down_left += 1,
            (false, false) => down_right += 1,
        }
    }
    (top_left, top_right, down_left, down_right)
}

fn run2(input_file: &Path) {
    // Preamble
    // println!("Space {SPACE_X}:{SPACE_Y} - Middle {MIDDLE_X}:{MIDDLE_Y}");

    // const SECONDS_TO_SIMULATE: usize = 1000;

    let file = File::open(input_file).unwrap();

    let mut robots: Vec<Robot> = Vec::new();

    // Parse
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        robots.push(line.into())
    }

    let mut i = 0;

    // let mut seen: HashMap<usize, Robot> = HashMap::new();
    // let mut max = 0;

    loop {
        i += 1;
        for robot in robots.iter_mut() {
            robot.tick();
        }

        if find_top_border(&robots) {
            break;
        }

        // let mirror_count = count_mirrors_robots(&robots);

        // if max < mirror_count{
        // println!("Max mirror count: {mirror_count}");
        // max = mirror_count;
        // print_grid_to_file(&robots, i);
        // }
        // if mirror_count > 80 {
        // print_grid_to_file(&robots, i + 1);
        // break;
        // }

        // if robots_mirror_tree(&robots, &mut seen) {
        //     let mirrored_ones: Vec<Robot> = seen.iter().map(|i| *i.1).collect();
        //     print_grid_to_file(&mirrored_ones, i);
        //     break;
        // }
    }

    // Solve
    println!("Result of part 2 is {}", i);
}

fn main() {
    let input_file = get_input_path(env!("CARGO_MANIFEST_DIR"));

    println!("Running {}", env!("CARGO_PKG_NAME"));
    println!("InputFile: {}", input_file.display());

    run(input_file.as_path());
    run2(input_file.as_path());
}

// fn print_grid(robots: &[Robot]) {
//     for y in 0..SPACE_Y {
//         for x in 0..SPACE_X {
//             let count = robots.iter().filter(|r| r.is_on(x, y)).count();
//             if count == 0 {
//                 print!(".");
//             } else {
//                 print!("{count}");
//             }
//         }
//         println!()
//     }
// }

// fn print_grid_to_file(robots: &[Robot], round: usize) {
//     let path = Path::new(env!("CARGO_MANIFEST_DIR"))
//         .join("..")
//         .join("target")
//         .join("d14")
//         .join(format!("{round:<03}.txt"));
//
//     std::fs::create_dir_all(path.parent().unwrap()).unwrap();
//
//     let mut writer = File::create_buffered(path).unwrap();
//
//     for y in 0..SPACE_Y {
//         for x in 0..SPACE_X {
//             if x == MIDDLE_X {
//                 writer.write("|".as_bytes()).unwrap();
//                 continue;
//             }
//             let count = robots.iter().filter(|r| r.is_on(x, y)).count();
//             if count == 0 {
//                 writer.write(".".as_bytes()).unwrap();
//             } else {
//                 writer.write(count.to_string().as_bytes()).unwrap();
//             }
//         }
//         writer.write("\n".as_bytes()).unwrap();
//     }
// }

fn find_top_border(robots: &Vec<Robot>) -> bool {
    'outer: for robot in robots {
        if robot.position_x > MIDDLE_X || robot.position_y > MIDDLE_Y {
            continue;
        }

        for new_x in robot.position_x + 1..robot.position_x + 10 {
            if !robots.iter().any(|r| r.is_on(new_x, robot.position_y)) {
                continue 'outer;
            }
        }
        return true;
    }
    false
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

// fn count_mirrors_robots(robots: &Vec<Robot>) -> usize {
//     let mut rtn = 0;
//     for robot in robots {
//         if MIDDLE_X == robot.position_x {
//             rtn += 1;
//         }
//         if MIDDLE_X < robot.position_x {
//             continue;
//         }
//
//         let distance = MIDDLE_X - robot.position_x;
//         let new_x = MIDDLE_X + distance;
//
//         if robots
//             .iter()
//             .enumerate()
//             .find(|r| r.1.is_on(new_x, robot.position_y))
//             .is_some()
//         {
//             rtn += 1;
//         }
//     }
//     rtn
// }

// fn robots_mirror_tree(robots: &Vec<Robot>, seen: &mut HashMap<usize, Robot>) -> bool {
//     let mut rtn = true;
//     for (i, robot) in robots.iter().enumerate() {
//         if MIDDLE_X > robot.position_x{
//             continue
//         }
//
//         // if MIDDLE_X.abs_diff(robot.position_x) as i64 >= robot.position_y {
//         //     continue
//         // }
//         if seen.contains_key(&i) {
//             continue;
//         }
//
//         // if MIDDLE_X == robot.position_x || MIDDLE_Y == robot.position_y {
//         //     continue;
//         // }
//
//         let is_left = robot.position_x < MIDDLE_X;
//
//         let new_x = if is_left {
//             let distance = MIDDLE_X - robot.position_x;
//             MIDDLE_X + distance
//         } else {
//             let distance = robot.position_x - MIDDLE_X;
//             MIDDLE_X - distance
//         };
//         // for x in 0..SPACE_X {
//         //     if new_x == x {
//         //         print!("n");
//         //         continue
//         //     }
//         //     if MIDDLE_X == x {
//         //         print!("|");
//         //         continue
//         //     }
//         //     if robot.position_x == x {
//         //         print!("r");
//         //         continue
//         //     }
//         //     print!(".");
//         // }
//         // println!();
//
//         if let Some((idx, inner_robot)) = robots
//             .iter()
//             .enumerate()
//             .find(|r| i != r.0 && r.1.is_on(new_x, robot.position_y))
//         {
//             if i == idx{
//                 println!("Fishy");
//             }
//             seen.insert(i, *robot);
//             seen.insert(idx, *inner_robot);
//         } else {
//             rtn = false;
//         }
//     }
//     rtn
// }
