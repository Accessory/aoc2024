use fxhash::{FxHashMap, FxHashSet};
use rayon::prelude::*;
use std::path::Path;
use utils::grid::Grid;
use utils::grid_direction::GridDirection;
use utils::grid_point::GridPoint;
use utils::{get_input_path, parse_into_char_vector_vector};

fn run(input_file: &Path) {
    // Preamble

    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let guard = grid.find_first(&'^').unwrap();
    let guard_direction = GridDirection::Up;

    // Solve
    let result = create_x_points(&grid, guard, guard_direction, max_x, max_y).len();

    // Result

    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Preamble
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();
    let max_x = grid.get_max_x();
    let max_y = grid.get_max_y();
    let guard = grid.find_first(&'^').unwrap();
    let guard_direction = GridDirection::Up;

    let points = create_x_points(&grid, guard, guard_direction, max_x, max_y);

    // Solve
    let result: usize = points
        .into_par_iter()
        .filter(|obstacle| {
            obstacle != &guard && has_cycle(&grid, guard, guard_direction, *obstacle, max_x, max_y)
        })
        .count();

    // Result
    println!("Result of part 2 is {}", result);
}

fn create_x_points(
    grid: &Grid<char>,
    mut guard: GridPoint,
    mut guard_direction: GridDirection,
    max_x: usize,
    max_y: usize,
) -> Vec<GridPoint> {
    let mut rtn: FxHashSet<GridPoint> = FxHashSet::default();
    loop {
        let _ = rtn.insert(guard);

        let next = match guard.next_by_direction_with_check(&guard_direction, max_x, max_y) {
            None => break,
            Some(value) => value,
        };

        let next_value = match grid.get_from_point(&next) {
            None => break,
            Some(value) => value,
        };

        match next_value {
            '#' | 'O' => guard_direction.turn_right(),
            '.' | '^' => guard = next,
            _ => panic!("Should not be here. Value is {next_value}"),
        }
    }

    rtn.into_iter().collect()
}

fn has_cycle(
    grid: &Grid<char>,
    mut guard: GridPoint,
    mut guard_direction: GridDirection,
    obstacle: GridPoint,
    max_x: usize,
    max_y: usize,
) -> bool {
    let mut seen: FxHashMap<GridPoint, u8> = FxHashMap::default();
    loop {
        let entry = seen.entry(guard).or_insert(0);
        if guard_direction.u8_has_direction(*entry) {
            return true;
        }
        *entry = guard_direction.add_to_u8(*entry);

        let next = match guard.next_by_direction_with_check(&guard_direction, max_x, max_y) {
            None => return false,
            Some(value) => value,
        };

        if next == obstacle {
            guard_direction.turn_right();
            continue;
        }

        let next_value = match grid.get_from_point(&next) {
            None => return false,
            Some(value) => value,
        };

        match next_value {
            '#' | 'O' => guard_direction.turn_right(),
            '.' | '^' => guard = next,
            _ => panic!("Should not be here. Value is {next_value}"),
        }
    }
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
