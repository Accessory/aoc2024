use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::LazyLock;
use utils::get_input_path;

#[derive(Debug, Default)]
struct Game {
    button_a_x: usize,
    button_a_y: usize,
    button_b_x: usize,
    button_b_y: usize,
    prize_x: usize,
    prize_y: usize,
}

enum ParsingState {
    ButtonA,
    ButtonB,
    Prize,
    Skip,
}

// #[derive(Debug,Default)]
// struct SearchContext {
//     a_press:usize,
//     b_press:usize,
//     cost: usize
// }

fn run(input_file: &Path) {
    // Preamble
    let mut parsing_state = ParsingState::ButtonA;
    let file = File::open(input_file).unwrap();
    // Parse
    let reader = BufReader::new(file);

    let mut games = Vec::new();

    let mut current_game = Game::default();
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        match parsing_state {
            ParsingState::ButtonA => {
                let (x, y) = get_button(line);
                current_game.button_a_x = x;
                current_game.button_a_y = y;
                parsing_state = ParsingState::ButtonB;
            }
            ParsingState::ButtonB => {
                let (x, y) = get_button(line);
                current_game.button_b_x = x;
                current_game.button_b_y = y;
                parsing_state = ParsingState::Prize;
            }
            ParsingState::Prize => {
                let (x, y) = get_prize(line);
                current_game.prize_x = x;
                current_game.prize_y = y;
                games.push(current_game);
                current_game = Game::default();
                parsing_state = ParsingState::Skip;
            }
            ParsingState::Skip => parsing_state = ParsingState::ButtonA,
        }
    }

    // Solve
    let mut result = 0;
    for game in games {
        // button_b_y(prize_x - button_a_x*a_p) = button_b_x(prize_y - button_a_y*a_p)
        // button_b_y * prize_x - (button_b_y*button_a_x)*ap =  button_b_x*prize_y * (button_b_x*button_a_y)*ap

        let bay_px = game.button_b_y * game.prize_x;
        let bbx_py = game.button_b_x * game.prize_y;

        let ra = game.button_b_y * game.button_a_x;
        let la = game.button_b_x * game.button_a_y;

        let t = bay_px.abs_diff(bbx_py);
        let d = ra.abs_diff(la);
        let a = t / d;

        if a > 100 {
            continue;
        }

        // button_a_x*a_p + button_b_x* b_p = prize_b
        let a_s = game.button_a_x * a;
        let r_y_p = game.prize_x.abs_diff(a_s);
        let b = r_y_p / game.button_b_x;

        if (a * game.button_a_x + b * game.button_b_x) != game.prize_x {
            // println!("Invalid X");
            continue;
        }

        if (a * game.button_a_y + b * game.button_b_y) != game.prize_y {
            // println!("Invalid Y");
            continue;
        }

        if a < 100 && b < 100 {
            // println!("A = {a} B = {b} Result = {}", 3 * a + b);
            result += 3 * a + b;
        }
        // break;
    }

    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    let mut parsing_state = ParsingState::ButtonA;
    let file = File::open(input_file).unwrap();
    // Parse
    let reader = BufReader::new(file);

    let mut games = Vec::new();

    let mut current_game = Game::default();
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        match parsing_state {
            ParsingState::ButtonA => {
                let (x, y) = get_button(line);
                current_game.button_a_x = x;
                current_game.button_a_y = y;
                parsing_state = ParsingState::ButtonB;
            }
            ParsingState::ButtonB => {
                let (x, y) = get_button(line);
                current_game.button_b_x = x;
                current_game.button_b_y = y;
                parsing_state = ParsingState::Prize;
            }
            ParsingState::Prize => {
                let (x, y) = get_prize(line);
                current_game.prize_x = x + 10000000000000;
                current_game.prize_y = y + 10000000000000;
                games.push(current_game);
                current_game = Game::default();
                parsing_state = ParsingState::Skip;
            }
            ParsingState::Skip => parsing_state = ParsingState::ButtonA,
        }
    }

    // Solve
    let mut result = 0;
    for game in games {
        // button_b_y(prize_x - button_a_x*a_p) = button_b_x(prize_y - button_a_y*a_p)
        // button_b_y * prize_x - (button_b_y*button_a_x)*ap =  button_b_x*prize_y * (button_b_x*button_a_y)*ap

        let bay_px = game.button_b_y * game.prize_x;
        let bbx_py = game.button_b_x * game.prize_y;

        let ra = game.button_b_y * game.button_a_x;
        let la = game.button_b_x * game.button_a_y;

        let t = bay_px.abs_diff(bbx_py);
        let d = ra.abs_diff(la);
        let a = t / d;

        // button_a_x*a_p + button_b_x* b_p = prize_b
        let a_s = game.button_a_x * a;
        let r_y_p = game.prize_x.abs_diff(a_s);
        let b = r_y_p / game.button_b_x;

        if (a * game.button_a_x + b * game.button_b_x) != game.prize_x {
            // println!("Invalid X");
            continue;
        }

        if (a * game.button_a_y + b * game.button_b_y) != game.prize_y {
            // println!("Invalid Y");
            continue;
        }

        // println!("A = {a} B = {b} Result = {}", 3 * a + b);
        result += 3 * a + b;
        // break;
    }

    // Result
    println!("Result of part 2 is {}", result);
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

static BUTTON_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"Button [AB]: X\+(\d+), Y\+(\d+)"#).unwrap());

static PRIZE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap());

fn get_button(line: String) -> (usize, usize) {
    let captures = BUTTON_REGEX.captures(&line).unwrap();
    let x = captures.get(1).unwrap().as_str().parse().unwrap();
    let y = captures.get(2).unwrap().as_str().parse().unwrap();
    (x, y)
}

fn get_prize(line: String) -> (usize, usize) {
    let captures = PRIZE_REGEX.captures(&line).unwrap();
    let x = captures.get(1).unwrap().as_str().parse().unwrap();
    let y = captures.get(2).unwrap().as_str().parse().unwrap();
    (x, y)
}
