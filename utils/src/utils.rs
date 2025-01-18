use std::ops::Add;

pub fn manhatten_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    x1.abs_diff(x2).add(y1.abs_diff(y2)) as i64
}

pub fn manhatten_distance_3d(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> i64 {
    x1.abs_diff(x2).add(y1.abs_diff(y2)).add(z1.abs_diff(z2)) as i64
}

pub fn manhatten_distance_3d_from_zero(x: i64, y: i64, z: i64) -> i64 {
    x.abs().add(y.abs()).add(z.abs())
}

pub fn split_number(number: u64, split_at: u64) -> (u64, u64) {
    let digits_separator = 10_u64.pow(split_at as u32);
    let right = number % digits_separator;
    let left = number / digits_separator;
    (left, right)
}

pub fn get_digits_count(mut number: u64) -> u64 {
    let mut rtn = 1;
    number = number / 10;
    loop {
        if number == 0 {
            break;
        }
        rtn += 1;
        number = number / 10;
    }
    rtn
}
