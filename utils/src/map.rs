use crate::point::MapPoint;

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Map {
    pub data: Vec<i64>,
    pub min_x: i64,
    pub max_x: i64,
    pub min_y: i64,
    pub max_y: i64,
    pub range_x: i64,
    pub range_y: i64,
}

impl Map {
    pub fn from_vec_vec(grid: Vec<Vec<i64>>) -> Self {
        let max_y = grid.len() as i64;
        let max_x = grid.iter().map(|i| i.len()).max().unwrap() as i64;

        Self {
            data: grid.into_iter().flatten().collect(),
            min_x: 0,
            max_x,
            min_y: 0,
            max_y,
            range_x: max_x,
            range_y: max_y,
        }
    }

    pub fn new(max_x: i64, max_y: i64) -> Self {
        Self {
            data: vec![0; (max_x * max_y) as usize],
            min_x: 0,
            max_x,
            min_y: 0,
            max_y,
            range_x: max_x,
            range_y: max_y,
        }
    }

    pub fn get_from_point(&self, point: &MapPoint) -> i64 {
        self.get(point.x, point.y)
    }

    pub fn get(&self, x: i64, y: i64) -> i64 {
        let mut rtn = y * self.range_y - self.min_y;
        rtn += x - self.min_x;
        *self.data.get(rtn as usize).unwrap()
    }

    pub fn set(&mut self, x: i64, y: i64, value: i64) {
        let mut rtn = y * self.range_y - self.min_y;
        rtn += x - self.min_x;
        self.data[rtn as usize] = value;
    }

    pub fn set_at_point(&mut self, point: &MapPoint, value: i64) {
        self.set(point.x, point.y, value);
    }

    pub fn is_point_in_map(&self, point: MapPoint) -> bool {
        self.is_in_map(point.x, point.y)
    }
    pub fn is_in_map(&self, x: i64, y: i64) -> bool {
        x >= self.min_x && y >= self.min_y && x < self.max_x && y < self.max_y
    }
}
