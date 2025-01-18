use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::AddAssign;
use std::path::Path;
use rayon::prelude::*;
use utils::grid::Grid;
use utils::grid_point::GridPoint;
use utils::{get_input_path, parse_into_char_vector_vector};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct SearchContext {
    position: GridPoint,
    steps: usize,
    previous_position: Vec<GridPoint>,
}

impl PartialOrd for SearchContext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchContext {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.previous_position.cmp(&other.previous_position))
    }
}

// impl SearchContext {
//     fn new(
//         position: GridPoint,
//         steps: usize,
//         previous_position: Vec<GridPoint>,
//         heuristic_value: usize
//     ) -> Self {
//         Self {
//             position,
//             steps,
//             previous_position,
//             heuristic_value,
//         }
//     }
// }

fn run(input_file: &Path) {
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();
    let start = grid.find_first(&'S').unwrap();
    let end = grid.find_first(&'E').unwrap();

    // Solve
    let mut queue = BTreeSet::new();

    queue.insert(SearchContext {
        position: start,
        ..Default::default()
    });

    let mut final_context = None;
    let mut seen = HashSet::new();

    while let Some(mut context) = queue.pop_first() {
        if end == context.position {
            final_context = Some(context);
            break;
        }

        if grid.data[context.position.y][context.position.x] == '#' {
            continue;
        }

        if !seen.insert(context.position) {
            continue;
        }

        let neighbors = context.position.generate_non_diagonal_neighbors();

        context.previous_position.push(context.position);
        let next = context.steps + 1;
        for neighbor in neighbors {
            queue.insert(SearchContext {
                position: neighbor,
                steps: next,
                previous_position: context.previous_position.clone(),
            });
        }
    }

    let mut context = final_context.unwrap();
    context.previous_position.push(end);

    // let mut results = Vec::new();
    //
    // for i in 0..context.previous_position.len() {
    //     for oi in i + 4..context.previous_position.len() {
    //         let position = &context.previous_position[i];
    //         let other = &context.previous_position[oi];
    //         let distance = position.manhatten_distance(other);
    //         if distance == 2 {
    //             results.push((oi - i - 2, *position, *other));
    //         }
    //     }
    // }

    let results:Vec<(usize, GridPoint, GridPoint)> =
        // for i in 0..context.previous_position.len() {
        (0..context.previous_position.len()).into_par_iter().map(
            |i|{
                let mut results = Vec::new();
                for oi in i + 3..context.previous_position.len() {
                    let position = &context.previous_position[i];
                    let other = &context.previous_position[oi];
                    let distance = position.manhatten_distance(other);
                    if distance <= 2 {
                        results.push((oi - i - 2, *position, *other));
                    }
                }
                results
            }
        ).flatten().collect();

    let mut results_time: HashMap<usize, usize> = HashMap::new();
    for result in results.iter() {
        results_time.entry(result.0).or_default().add_assign(1);
    }

    // let mut result = 0;
    // for i in results_time {
    //     if i.1 == 1 {
    //         println!("There is one cheats that save {} picoseconds.", i.0);
    //     } else {
    //         println!("There are {} cheats that save {} picoseconds.", i.1, i.0);
    //     }
    //     if i.0 >= 100 {
    //         result += i.1;
    //     }
    // }

    // Result
    let result:usize = results_time.iter().filter_map(|x| {
        if *x.0 >= 100 {
            Some(x.1)
        } else {
            None
        }
    }).sum();
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &Path) {
    // Parse
    let grid: Grid<char> = parse_into_char_vector_vector(input_file).into();
    let start = grid.find_first(&'S').unwrap();
    let end = grid.find_first(&'E').unwrap();

    // Solve
    let mut queue = BTreeSet::new();

    queue.insert(SearchContext {
        position: start,
        ..Default::default()
    });

    let mut final_context = None;
    let mut seen = HashSet::new();

    while let Some(mut context) = queue.pop_first() {
        if end == context.position {
            final_context = Some(context);
            break;
        }

        if grid.data[context.position.y][context.position.x] == '#' {
            continue;
        }

        if !seen.insert(context.position) {
            continue;
        }

        let neighbors = context.position.generate_non_diagonal_neighbors();

        context.previous_position.push(context.position);
        let next = context.steps + 1;
        for neighbor in neighbors {
            queue.insert(SearchContext {
                position: neighbor,
                steps: next,
                previous_position: context.previous_position.clone(),
            });
        }
    }

    let mut context = final_context.unwrap();
    context.previous_position.push(end);

    let results:Vec<(usize, GridPoint, GridPoint)> =
    // for i in 0..context.previous_position.len() {
    (0..context.previous_position.len()).into_par_iter().map(
        |i|{
            let mut results = Vec::new();
            for oi in i + 50..context.previous_position.len() {
                let position = &context.previous_position[i];
                let other = &context.previous_position[oi];
                let distance = position.manhatten_distance(other);
                if distance <= 20 {
                    results.push((oi - i - distance, *position, *other));
                }
            }
            results
        }
    ).flatten().collect();

    let mut results_time: HashMap<usize, usize> = HashMap::new();
    for result in results {
        results_time.entry(result.0).or_default().add_assign(1);
    }

    // let mut result = 0;
    // for i in results_time {
    //     if i.0 < 50 {
    //         continue;
    //     }
    //     if i.1 == 1 {
    //         println!("There is one cheats that save {} picoseconds.", i.0);
    //     } else {
    //         println!("There are {} cheats that save {} picoseconds.", i.1, i.0);
    //     }
    //     if i.0 >= 100 {
    //         result += i.1;
    //     }
    // }

    let result:usize = results_time.iter().filter_map(|x| {
        if *x.0 >= 100 {
            Some(x.1)
        } else {
            None
        }
    }).sum();

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
