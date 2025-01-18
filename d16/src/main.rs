use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::path::Path;

use utils::grid::Grid;
use utils::grid_direction::GridDirection;
use utils::grid_point::GridPoint;
use utils::{get_input_path, parse_into_char_vector_vector};

struct SearchContext {
    position: GridPoint,
    direction: GridDirection,
    walked: Vec<GridPoint>,
    points: usize,
    turns: usize,
}

impl Eq for SearchContext {}

impl PartialEq<Self> for SearchContext {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
            && self.direction == other.direction
            && self.position == self.position
    }
}

impl PartialOrd<Self> for SearchContext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchContext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.points
            .cmp(&other.points)
            .then_with(|| self.direction.cmp(&other.direction))
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl SearchContext {
    pub(crate) fn next_forward(&self) -> SearchContext {
        let mut new_walked = self.walked.clone();
        new_walked.push(self.position);
        SearchContext {
            position: self.position.next_by_direction(&self.direction),
            direction: self.direction,
            walked: new_walked,
            points: self.points + 1,
            turns: self.turns,
        }
    }
    pub(crate) fn next_turn_left(&self) -> SearchContext {
        SearchContext {
            position: self.position,
            direction: self.direction.left(),
            walked: self.walked.clone(),
            points: self.points + 1000,
            turns: self.turns + 1,
        }
    }
    pub(crate) fn next_turn_right(&self) -> SearchContext {
        SearchContext {
            position: self.position,
            direction: self.direction.right(),
            walked: self.walked.clone(),
            points: self.points + 1000,
            turns: self.turns + 1,
        }
    }
}

fn run(input_file: &Path) {
    // Preamble
    let mut results = Vec::new();

    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    // Solve
    let start = grid.find_first(&'S').unwrap();
    let end = grid.find_first(&'E').unwrap();

    let mut queue = BTreeSet::new();

    queue.insert(SearchContext {
        position: start,
        direction: GridDirection::Right,
        walked: vec![],
        points: 0,
        turns: 0,
    });

    let mut seen: HashMap<(GridPoint, GridDirection), usize> = HashMap::new();

    while let Some(item) = queue.pop_first() {
        if item.position == end {
            results.push(item);
            break;
        }

        if grid.data[item.position.y][item.position.x] == '#' {
            continue;
        }

        let key = (item.position, item.direction);

        // if seen.get(&key).is_some()
        if seen.get(&key).is_some_and(|o| o < &item.points) {
            continue;
        } else {
            seen.insert(key, item.points);
        }

        let next_forward = item.next_forward();
        let next_turn_left = item.next_turn_left();
        let next_turn_right = item.next_turn_right();

        queue.insert(next_forward);
        queue.insert(next_turn_left);
        queue.insert(next_turn_right);

        // println!("Order");
        // for item in queue.iter() {
        //     println!(
        //         "Direction: {}, Position: {:?}, Points: {}",
        //         item.direction, item.position, item.points
        //     );
        // }
        // println!();
    }

    // Result
    let result = results.iter().min_by_key(|s| s.points).unwrap();
    println!(
        "Resulting Path has {} turns and took {} steps.",
        result.turns,
        result.walked.len()
    );
    println!("Result of part 1 is {}", result.points);
}

fn run2(input_file: &Path) {
    // Preamble
    let mut results = Vec::new();

    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();

    // Solve

    let start = grid.find_first(&'S').unwrap();
    let end = grid.find_first(&'E').unwrap();

    let mut queue = VecDeque::new();

    queue.push_back(SearchContext {
        position: start,
        direction: GridDirection::Right,
        walked: vec![],
        points: 0,
        turns: 0,
    });

    let mut seen: HashMap<(GridPoint, GridDirection), usize> = HashMap::new();
    let mut min = usize::MAX;

    while let Some(item) = queue.pop_front() {
        if item.points > min {
            continue;
        }

        if item.position == end {
            min = item.points.min(min);
            results.push(item);
            continue;
        }

        if grid.data[item.position.y][item.position.x] == '#' {
            continue;
        }

        let key = (item.position, item.direction);

        if seen.get(&key).is_some_and(|o| o < &item.points) {
            continue;
        } else {
            seen.insert(key, item.points);
        }

        let next_forward = item.next_forward();
        let next_turn_left = item.next_turn_left();
        let next_turn_right = item.next_turn_right();

        queue.push_front(next_forward);
        queue.push_back(next_turn_left);
        queue.push_back(next_turn_right);
    }

    // Result
    let mut result_tiles = HashSet::new();
    result_tiles.insert(start);
    result_tiles.insert(end);
    for result in results {
        if result.points != min {
            continue;
        }
        result.walked.into_iter().for_each(|i| {
            result_tiles.insert(i);
        })
    }

    println!("Result of part 2 is {}", result_tiles.len());
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
        run(&get_test_input_2_path(env!("CARGO_MANIFEST_DIR")));
    }

    #[test]
    fn test_input_part_2() {
        run2(&get_test_input_path(env!("CARGO_MANIFEST_DIR")));
        run2(&get_test_input_2_path(env!("CARGO_MANIFEST_DIR")));
    }
}
