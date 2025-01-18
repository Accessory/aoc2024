use std::fs;
use std::path::Path;
use utils::get_input_path;

fn run(input_file: &Path) {
    // Preamble
    const EMPTY_SPACE: u16 = 0xFFFF;

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let mut disk: Vec<u16> = Vec::with_capacity(line.len() * 5);

    // Solve
    let mut file_id = 0;
    let mut is_file = true;
    for c in line.chars() {
        let number = c.to_digit(10).unwrap();
        if is_file {
            (0..number).for_each(|_| disk.push(file_id));
            file_id += 1;
        } else {
            (0..number).for_each(|_| disk.push(EMPTY_SPACE));
        }

        is_file = !is_file;
    }

    let mut last_position = disk.len() - 1;
    for pos in 0..disk.len() {
        if pos >= last_position {
            break;
        }
        if disk[pos] == EMPTY_SPACE {
            disk.swap(pos, last_position);
            loop {
                last_position -= 1;
                if disk[last_position] != EMPTY_SPACE {
                    break;
                }
            }
        }
    }
    let mut result: u64 = 0;
    for (i, disk_space) in disk.iter().enumerate() {
        if *disk_space == EMPTY_SPACE {
            break;
        }

        result += (i * (*disk_space) as usize) as u64;
    }

    // Result
    println!("Result of part 1 is {}", result);
}

#[derive(Debug, Copy, Clone)]
struct DiskSpace {
    file_id: Option<u16>,
    size: u16,
}

fn run2(input_file: &Path) {
    // Preamble
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let mut disk: Vec<DiskSpace> = Vec::with_capacity(line.len() * 2);

    // Solve
    let mut file_id = 0;
    let mut is_file = true;
    for c in line.chars() {
        let number = c.to_digit(10).unwrap();
        if is_file {
            disk.push(DiskSpace {
                file_id: Some(file_id),
                size: number as u16,
            });
            file_id += 1;
        } else {
            disk.push(DiskSpace {
                file_id: None,
                size: number as u16,
            });
        }

        is_file = !is_file;
    }

    let mut last_file_position = disk.len();
    for file_id in (0..file_id).rev() {
        let (file_position, _) = disk[0..last_file_position]
            .iter()
            .enumerate()
            .rfind(|(_file_position, f)| f.file_id.is_some_and(|fid| fid == file_id))
            .expect("Could not find the disk part. This should not happen");

        last_file_position = file_position;

        let free_space: Option<usize> =
            disk[0..file_position]
                .iter()
                .enumerate()
                .find_map(|(i, ds)| {
                    if ds.file_id.is_none() && ds.size >= disk[file_position].size {
                        Some(i)
                    } else {
                        None
                    }
                });

        if let Some(free_space_position) = free_space {
            if disk[file_position].size == disk[free_space_position].size {
                disk.swap(file_position, free_space_position);
                // disk.remove(file_position);
            } else {
                disk[free_space_position].size -= disk[file_position].size;
                let file = disk[file_position];
                disk[file_position].file_id = None;
                disk.insert(free_space_position, file);
            }
        }
    }

    // for ds in disk {
    //     for _ in 0..ds.size {
    //         if let Some(d) = ds.file_id {
    //             print!("{}", d);
    //         } else {
    //             print!(".");
    //         }
    //     }
    // }

    let mut result: u64 = 0;
    let mut positions = 0;
    for ds in disk {
        for _ in 0..ds.size {
            if let Some(d) = ds.file_id {
                result += positions as u64 * d as u64;
            }
            positions += 1;
        }
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
