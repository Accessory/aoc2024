use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

pub mod grid;
pub mod grid_direction;
pub mod grid_point;
pub mod hash_point_map;
pub mod map;
pub mod map_direction;
pub mod point;
pub mod utils;
pub mod vector3;

pub fn parse_file_into<T>(input_file: &Path) -> Vec<T>
where
    T: From<String>,
{
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap().into())
        .collect::<Vec<T>>()
}

pub fn parse_into_usize_vector(input_file: &Path) -> Vec<usize> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .parse::<usize>()
                .expect("Could not parse \"{l}\" into usize")
        })
        .collect::<Vec<usize>>()
}

pub fn parse_into_i64_vector(input_file: &str) -> Vec<i64> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .parse::<i64>()
                .expect("Could not parse \"{l}\" into i64")
        })
        .collect::<Vec<i64>>()
}

pub fn get_input_path(cargo_manifest_dir_path: &str) -> PathBuf {
    Path::new(cargo_manifest_dir_path)
        .join("input")
        .join("input.txt")
}

pub fn get_test_input_path(src_path: &str) -> PathBuf {
    Path::new(src_path).join("input").join("input_test.txt")
}

pub fn get_test_input_2_path(src_path: &str) -> PathBuf {
    Path::new(src_path).join("input").join("input_test_2.txt")
}

pub fn get_test_input_e_path(src_path: &str) -> PathBuf {
    Path::new(src_path).join("input").join("input_test_e.txt")
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (x.min(y), x.max(y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn parse_into_char_vector_vector(input_file: &Path) -> Vec<Vec<char>> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut rtn = Vec::new();

    for line in reader.lines() {
        rtn.push(line.unwrap().trim().chars().collect())
    }

    rtn
}

/// The function tries to parse every char to a digit. If the char cannot be converted into a digit
/// it will be transformed into 0xFF instead
pub fn parse_into_u8_vector_vector(input_file: &Path) -> Vec<Vec<u8>> {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut rtn = Vec::new();

    for line in reader.lines() {
        rtn.push(
            line.unwrap()
                .trim()
                .chars()
                .map(|i| i.to_digit(10).unwrap_or(0xFF) as u8)
                .collect(),
        )
    }

    rtn
}
