use std::collections::HashMap;
use std::ops::AddAssign;
use std::path::Path;

use utils::grid::Grid;
use utils::grid_point::GridPoint;
use utils::{get_input_path, parse_into_char_vector_vector};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct SearchContext {
    position: GridPoint,
    previous_position: Vec<GridPoint>,
}

fn run(input_file: &Path) {
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();
    let start = grid.find_first(&'S').unwrap();
    let end = grid.find_first(&'E').unwrap();

    // Solve
    let mut context = SearchContext {
        position: start,
        ..Default::default()
    };
    let mut last = GridPoint::default();

    'outer_loop: loop {
        if end == context.position {
            break;
        }

        let neighbors = context.position.generate_non_diagonal_neighbors();

        context.previous_position.push(context.position);
        for neighbor in neighbors {
            if grid.data[neighbor.y][neighbor.x] == '#' {
                continue;
            }
            if last == neighbor {
                continue;
            }
            last = context.position;
            context.position = neighbor;
            continue 'outer_loop;
        }
    }

    context.previous_position.push(end);

    let mut results = Vec::new();

    for i in 0..context.previous_position.len() {
        for oi in i + 4..context.previous_position.len() {
            let position = &context.previous_position[i];
            let other = &context.previous_position[oi];
            let distance = position.manhatten_distance(other);
            if distance == 2 {
                results.push(oi - i - 2);
            }
        }
    }

    let mut results_time: HashMap<usize, usize> = HashMap::new();
    for result in results {
        results_time.entry(result).or_default().add_assign(1);
    }

    // Result
    let result: usize = results_time
        .iter()
        .filter_map(|x| if *x.0 >= 100 { Some(x.1) } else { None })
        .sum();
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();
    let start = grid.find_first(&'S').unwrap();
    let end = grid.find_first(&'E').unwrap();

    // Solve
    let mut context = SearchContext {
        position: start,
        ..Default::default()
    };
    let mut last = GridPoint::default();

    'outer_loop: loop {
        if end == context.position {
            break;
        }

        let neighbors = context.position.generate_non_diagonal_neighbors();

        context.previous_position.push(context.position);
        for neighbor in neighbors {
            if grid.data[neighbor.y][neighbor.x] == '#' {
                continue;
            }
            if last == neighbor {
                continue;
            }
            last = context.position;
            context.position = neighbor;
            continue 'outer_loop;
        }
    }

    context.previous_position.push(end);

    let mut results = Vec::new();

    for i in 0..context.previous_position.len() {
        for oi in i + 50..context.previous_position.len() {
            let position = &context.previous_position[i];
            let other = &context.previous_position[oi];
            let distance = position.manhatten_distance(other);
            if distance <= 20 {
                results.push(oi - i - distance);
            }
        }
    }

    let mut results_time: HashMap<usize, usize> = HashMap::new();
    for result in results {
        results_time.entry(result).or_default().add_assign(1);
    }

    let result: usize = results_time
        .iter()
        .filter_map(|x| if *x.0 >= 100 { Some(x.1) } else { None })
        .sum();

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
