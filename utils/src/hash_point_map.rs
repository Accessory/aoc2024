use std::{collections::HashMap, fmt::Display};

use crate::point::MapPoint;

#[derive(Debug, Default, Clone)]
pub struct HashPointMap<T> {
    pub data: HashMap<MapPoint, T>,
}

impl<T> HashPointMap<T> {
    pub fn new(data: HashMap<MapPoint, T>) -> Self {
        Self { data }
    }

    pub fn get(&self, point: &MapPoint) -> Option<&T> {
        self.data.get(point)
    }

    pub fn get_mut(&mut self, point: &MapPoint) -> Option<&mut T> {
        self.data.get_mut(point)
    }

    pub fn push(&mut self, key: MapPoint, value: T) {
        self.data.insert(key, value);
    }

    pub fn insert(&mut self, key: MapPoint, value: T) -> Option<T> {
        self.data.insert(key, value)
    }

    pub fn remove(&mut self, point: &MapPoint) -> Option<T> {
        self.data.remove(point)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get_min_x_max_x_min_y_max_y(&self) -> (i64, i64, i64, i64) {
        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;

        for (map_point, _) in self.data.iter() {
            min_x = min_x.min(map_point.x);
            max_x = max_x.max(map_point.x);
            min_y = min_y.min(map_point.y);
            max_y = max_y.max(map_point.y);
        }

        (min_x, max_x, min_y, max_y)
    }

    pub fn print_all_with_offset(&self, fill: char, offset: i64)
    where
        T: Display,
    {
        let (min_x, max_x, min_y, max_y) = self.get_min_x_max_x_min_y_max_y();
        self.print_from_to(
            min_x - offset,
            max_x + offset,
            min_y - offset,
            max_y + offset,
            fill,
        );
    }

    pub fn print_all(&self, fill: char)
    where
        T: Display,
    {
        let (min_x, max_x, min_y, max_y) = self.get_min_x_max_x_min_y_max_y();
        self.print_from_to(min_x, max_x, min_y, max_y, fill);
    }

    pub fn print_from_to(&self, min_x: i64, max_x: i64, min_y: i64, max_y: i64, fill: char)
    where
        T: Display,
    {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let current_point = MapPoint { x, y };
                match self.data.get(&current_point) {
                    Some(value) => print!("{value}"),
                    None => print!("{fill}"),
                }
            }
            println!()
        }
    }

    pub fn get_x_y(&self, x: i64, y: i64) -> Option<&T> {
        let point = MapPoint { x, y };
        self.get(&point)
    }
}
