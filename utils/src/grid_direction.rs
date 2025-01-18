use crate::grid_point::GridPoint;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GridDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GridCorner {
    TopLeft,
    TopRight,
    DownRight,
    DownLeft,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridCornerPosition {
    pub corner: GridCorner,
    pub position: GridPoint,
}

impl From<&char> for GridDirection {
    fn from(value: &char) -> Self {
        match *value {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("Char {value} unkown."),
        }
    }
}

impl From<&mut char> for GridDirection {
    fn from(value: &mut char) -> Self {
        match *value {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("Char {value} unkown."),
        }
    }
}

impl GridDirection {
    pub fn to_u8(&self) -> u8 {
        match self {
            GridDirection::Up => 0b0000_0001,
            GridDirection::Right => 0b0000_0010,
            GridDirection::Down => 0b0000_0100,
            GridDirection::Left => 0b0000_1000,
        }
    }
    pub fn to_char_u8(&self) -> u8 {
        match self {
            GridDirection::Up => b'^',
            GridDirection::Right => b'>',
            GridDirection::Down => b'v',
            GridDirection::Left => b'<',
        }
    }

    pub fn u8_has_direction(&self, byte: u8) -> bool {
        self.to_u8() & byte != 0
    }

    pub fn add_to_u8(&self, byte: u8) -> u8 {
        self.to_u8() | byte
    }

    pub fn get_directions() -> [GridDirection; 4] {
        [
            GridDirection::Up,
            GridDirection::Right,
            GridDirection::Down,
            GridDirection::Left,
        ]
    }

    pub fn get_int_char(&self) -> char {
        match self {
            GridDirection::Up => '0',
            GridDirection::Right => '1',
            GridDirection::Down => '2',
            GridDirection::Left => '3',
        }
    }

    pub fn from_int_char(c: char) -> Self {
        match c {
            '0' => GridDirection::Up,
            '1' => GridDirection::Right,
            '2' => GridDirection::Down,
            '3' => GridDirection::Left,
            _ => panic!("Not a valid direction {c}"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            GridDirection::Up => '^',
            GridDirection::Right => '>',
            GridDirection::Down => 'v',
            GridDirection::Left => '<',
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            GridDirection::Up => GridDirection::Right,
            GridDirection::Right => GridDirection::Down,
            GridDirection::Down => GridDirection::Left,
            GridDirection::Left => GridDirection::Up,
        }
    }

    pub fn turn_left(&mut self) {
        *self = match self {
            GridDirection::Up => GridDirection::Left,
            GridDirection::Right => GridDirection::Up,
            GridDirection::Down => GridDirection::Right,
            GridDirection::Left => GridDirection::Down,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            GridDirection::Up => GridDirection::Right,
            GridDirection::Right => GridDirection::Down,
            GridDirection::Down => GridDirection::Left,
            GridDirection::Left => GridDirection::Up,
        }
    }

    pub fn left(&self) -> Self {
        match self {
            GridDirection::Up => GridDirection::Left,
            GridDirection::Right => GridDirection::Up,
            GridDirection::Down => GridDirection::Right,
            GridDirection::Left => GridDirection::Down,
        }
    }
}

impl Display for GridDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GridDirection::Up => write!(f, "^"),
            GridDirection::Right => write!(f, ">"),
            GridDirection::Down => write!(f, "v"),
            GridDirection::Left => write!(f, "<"),
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Grid8WayDirection {
    LeftUp,
    Up,
    RightUp,
    Right,
    RightDown,
    Down,
    LeftDown,
    Left,
}

impl Grid8WayDirection {
    pub fn get_all_directions_array() -> [Grid8WayDirection; 8] {
        [
            Grid8WayDirection::LeftUp,
            Grid8WayDirection::Up,
            Grid8WayDirection::RightUp,
            Grid8WayDirection::Right,
            Grid8WayDirection::RightDown,
            Grid8WayDirection::Down,
            Grid8WayDirection::LeftDown,
            Grid8WayDirection::Left,
        ]
    }
}

impl Display for Grid8WayDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Grid8WayDirection::LeftUp => {
                write!(f, "LeftUp")
            }
            Grid8WayDirection::Up => {
                write!(f, "Up")
            }
            Grid8WayDirection::RightUp => {
                write!(f, "RightUp")
            }
            Grid8WayDirection::Right => {
                write!(f, "Right")
            }
            Grid8WayDirection::RightDown => {
                write!(f, "RightDown")
            }
            Grid8WayDirection::Down => {
                write!(f, "Down")
            }
            Grid8WayDirection::LeftDown => {
                write!(f, "LeftDown")
            }
            Grid8WayDirection::Left => {
                write!(f, "Left")
            }
        }
    }
}
