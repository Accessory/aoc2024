#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MapDirection {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for MapDirection {
    fn from(value: char) -> Self {
        match value {
            'u' | 'U' => Self::Up,
            'r' | 'R' => Self::Right,
            'd' | 'D' => Self::Down,
            'l' | 'L' => Self::Left,
            _ => panic!("Char {value} does not produce a direction."),
        }
    }
}

impl MapDirection {
    pub fn get_char_direction(&self) -> char {
        match self {
            MapDirection::Up => '^',
            MapDirection::Right => '>',
            MapDirection::Down => 'v',
            MapDirection::Left => '<',
        }
    }
    pub fn get_directions() -> [MapDirection; 4] {
        [
            MapDirection::Up,
            MapDirection::Right,
            MapDirection::Down,
            MapDirection::Left,
        ]
    }
}
