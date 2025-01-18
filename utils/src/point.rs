use crate::{map_direction::MapDirection, utils::manhatten_distance};

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct MapPoint {
    pub x: i64,
    pub y: i64,
}

impl MapPoint {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn diff(&self, other: &Self) -> (i64, i64) {
        let x = self.x - other.x;
        let y = self.y - other.y;
        (x, y)
    }

    pub fn add(&self, other_x: i64, other_y: i64) -> (i64, i64) {
        let x = self.x - other_x;
        let y = self.y - other_y;
        (x, y)
    }

    pub fn move_by_direction(&mut self, direction: &MapDirection) {
        match direction {
            MapDirection::Up => self.move_up(),
            MapDirection::Right => self.move_right(),
            MapDirection::Down => self.move_down(),
            MapDirection::Left => self.move_left(),
        };
    }

    // pub fn in_on_line(&self, start: &MapPoint, end: &MapPoint) -> bool {
    //     return distance(start, self) + distance(self, end) == distance(start, end);
    //     // return round(distance(start, self)) + round(distance(self, end)) == round(distance(start, end));
    // }

    pub fn slop(&self, other: &MapPoint) -> f64 {
        f64::atan2((self.x - other.x) as f64, (self.y - other.y) as f64)
    }

    // pub fn distance(&self, other: &MapPoint) -> f64{
    //     distance(self, other)
    // }
}

// fn round(start: f64) -> f64 {
//     (start * 1.0).round() /1.0
// }

// fn distance(start: &MapPoint, end: &MapPoint) -> f64 {
//     f64::sqrt(
//         (start.x as f64 - end.x as f64).powi(2)
//             + (start.y as f64 - end.y as f64).powi(2),
//     )
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MapWalker {
    pub direction: MapDirection,
    pub position: MapPoint,
}

impl From<MapDirection> for MapWalker {
    fn from(direction: MapDirection) -> Self {
        Self {
            direction,
            position: MapPoint { x: 0, y: 0 },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapInfoWalker<T> {
    pub map_walker: MapWalker,
    pub info: T,
}

impl<T> MapInfoWalker<T> {
    pub fn r#move(&mut self) {
        self.map_walker.r#move();
    }
}

impl MapWalker {
    pub fn r#move(&mut self) {
        match self.direction {
            MapDirection::Up => self.position.move_up(),
            MapDirection::Right => self.position.move_right(),
            MapDirection::Down => self.position.move_down(),
            MapDirection::Left => self.position.move_left(),
        };
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            MapDirection::Up => MapDirection::Left,
            MapDirection::Right => MapDirection::Up,
            MapDirection::Down => MapDirection::Right,
            MapDirection::Left => MapDirection::Down,
        }
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            MapDirection::Up => MapDirection::Right,
            MapDirection::Right => MapDirection::Down,
            MapDirection::Down => MapDirection::Left,
            MapDirection::Left => MapDirection::Up,
        }
    }

    pub fn turn_around(&mut self) {
        self.direction = match self.direction {
            MapDirection::Up => MapDirection::Down,
            MapDirection::Right => MapDirection::Left,
            MapDirection::Down => MapDirection::Up,
            MapDirection::Left => MapDirection::Right,
        }
    }

    pub fn move_up(&mut self) {
        self.position.move_up()
    }

    pub fn move_down(&mut self) {
        self.position.move_down()
    }

    pub fn move_left(&mut self) {
        self.position.move_left()
    }

    pub fn move_right(&mut self) {
        self.position.move_right()
    }

    pub fn next_up(&self) -> MapPoint {
        self.position.next_up()
    }

    pub fn next_down(&self) -> MapPoint {
        self.position.next_down()
    }

    pub fn next_left(&self) -> MapPoint {
        self.position.next_left()
    }

    pub fn next_right(&self) -> MapPoint {
        self.position.next_right()
    }

    pub fn next(&self) -> MapPoint {
        self.position.next_from_direction(&self.direction)
    }
}

impl MapPoint {
    pub fn generate_non_diagonal_neigbors(&self) -> Vec<MapPoint> {
        vec![
            MapPoint {
                x: self.x - 1,
                y: self.y,
            },
            MapPoint {
                x: self.x,
                y: self.y - 1,
            },
            MapPoint {
                x: self.x,
                y: self.y + 1,
            },
            MapPoint {
                x: self.x + 1,
                y: self.y,
            },
        ]
    }
    pub fn generate_neighbors(&self) -> Vec<MapPoint> {
        vec![
            MapPoint {
                x: self.x - 1,
                y: self.y - 1,
            },
            MapPoint {
                x: self.x - 1,
                y: self.y,
            },
            MapPoint {
                x: self.x - 1,
                y: self.y + 1,
            },
            MapPoint {
                x: self.x,
                y: self.y - 1,
            },
            MapPoint {
                x: self.x,
                y: self.y + 1,
            },
            MapPoint {
                x: self.x + 1,
                y: self.y - 1,
            },
            MapPoint {
                x: self.x + 1,
                y: self.y,
            },
            MapPoint {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn next_from_direction(&self, direction: &MapDirection) -> Self {
        match direction {
            MapDirection::Up => self.next_up(),
            MapDirection::Right => self.next_right(),
            MapDirection::Down => self.next_down(),
            MapDirection::Left => self.next_left(),
        }
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_right_down(&mut self) {
        self.x += 1;
        self.y -= 1;
    }

    pub fn move_down_right(&mut self) {
        self.x += 1;
        self.y -= 1;
    }

    pub fn manhatten_distance_from_x_y(&self, x: i64, y: i64) -> i64 {
        manhatten_distance(self.x, self.y, x, y)
    }

    pub fn manhatten_distance(&self, point: &Self) -> i64 {
        manhatten_distance(self.x, self.y, point.x, point.y)
    }

    pub fn next_up(&self) -> MapPoint {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn next_left(&self) -> MapPoint {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn next_down(&self) -> MapPoint {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn next_right(&self) -> MapPoint {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}
