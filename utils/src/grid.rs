use std::fmt::{Display, Formatter};

use crate::grid_point::GridPoint;

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid_to_string())
    }
}

impl From<Vec<Vec<char>>> for Grid<char> {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self { data: value }
    }
}

impl From<Vec<Vec<u8>>> for Grid<u8> {
    fn from(value: Vec<Vec<u8>>) -> Self {
        Self { data: value }
    }
}

impl<T> Grid<T>
where
    T: Eq,
{
    pub fn find_first(&self, to_find: &T) -> Option<GridPoint> {
        for (y, row) in self.data.iter().enumerate() {
            for (x, column) in row.iter().enumerate() {
                if column == to_find {
                    return Some(GridPoint { x, y });
                }
            }
        }
        None
    }

    pub fn count_for(&self, value: &T) -> usize {
        self.data.iter().flatten().filter(|i| *i == value).count()
    }
}

impl<T> Grid<T> {
    pub fn with_width_height(width: usize, height: usize, fill: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![vec![fill; width]; height],
        }
    }

    pub fn get_max_x(&self) -> usize {
        self.data.iter().map(|i| i.len()).max().unwrap_or(0)
    }

    pub fn get_max_y(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|i| i.get(x))
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y][x] = value;
    }

    pub fn get_from_point(&self, point: &GridPoint) -> Option<&T> {
        self.get(point.x, point.y)
    }

    pub fn set_from_point(&mut self, point: &GridPoint, value: T) {
        self.set(point.x, point.y, value)
    }

    pub fn create_sub_grid(&self, x: usize, y: usize, width: usize, height: usize) -> Self
    where
        T: Copy,
    {
        let mut new_data: Vec<Vec<T>> = Vec::with_capacity(height);
        for y in y..y + width {
            let mut row = Vec::with_capacity(width);
            for x in x..x + width {
                let point = self.get(x, y).unwrap();
                row.push(*point);
            }
            new_data.push(row);
        }
        Grid { data: new_data }
    }

    pub fn get_sub_grid(&self, x: usize, y: usize, width: usize, height: usize) -> Grid<&T> {
        let mut new_data: Vec<Vec<&T>> = Vec::with_capacity(height);
        for y in y..y + width {
            let mut row = Vec::with_capacity(width);
            for x in x..x + width {
                let point = self.get(x, y).unwrap();
                row.push(point);
            }
            new_data.push(row);
        }
        Grid { data: new_data }
    }

    pub fn print_data(&self)
    where
        T: Display,
    {
        for row in self.data.iter() {
            for i in row.iter() {
                print!("{}", *i);
            }
            println!();
        }
    }

    pub fn grid_to_string(&self) -> String
    where
        T: Display,
    {
        let mut rtn = String::with_capacity(self.get_max_x() * self.get_max_y() + self.get_max_y());
        for row in self.data.iter() {
            for i in row.iter() {
                rtn.push_str(&format!("{}", *i));
            }
            rtn.push('\n');
        }
        rtn
    }
}
