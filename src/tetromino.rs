use crate::Rgb;

#[derive(Clone, Debug, PartialEq)]
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate(&self, diff: &DirectionDiff) -> Self {
        match (self, diff) {
            (Direction::Up, DirectionDiff::Cw) => Self::Right,
            (Direction::Up, DirectionDiff::Ccw) => Self::Left,
            (Direction::Right, DirectionDiff::Cw) => Self::Down,
            (Direction::Right, DirectionDiff::Ccw) => Self::Up,
            (Direction::Down, DirectionDiff::Cw) => Self::Left,
            (Direction::Down, DirectionDiff::Ccw) => Self::Right,
            (Direction::Left, DirectionDiff::Cw) => Self::Up,
            (Direction::Left, DirectionDiff::Ccw) => Self::Down,
        }
    }
}

pub enum DirectionDiff {
    Cw,
    Ccw,
}

impl Tetromino {
    pub fn random() -> Self {
        let v: u8 = rand::random();
        match v % 7 {
            0 => Self::I,
            1 => Self::J,
            2 => Self::L,
            3 => Self::O,
            4 => Self::S,
            5 => Self::T,
            6 => Self::Z,
            _ => unreachable!("v%7 is always in range 0..=6"),
        }
    }

    const fn color(&self) -> Rgb {
        match self {
            Self::I => Rgb(0, 255, 255),
            Self::J => Rgb(0, 0, 255),
            Self::L => Rgb(255, 128, 0),
            Self::O => Rgb(255, 255, 0),
            Self::S => Rgb(0, 255, 0),
            Self::T => Rgb(255, 0, 255),
            Self::Z => Rgb(255, 0, 0),
        }
    }

    pub fn direction_pattern(&self, direction: &Direction) -> [[bool; 4]; 4] {
        let idx = match direction {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        };

        self.directions()[idx]
    }

    fn directions(&self) -> [[[bool; 4]; 4]; 4] {
        let dir = match self {
            Self::I => [
                [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
                [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
                [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
            ],
            Self::J => [
                [[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0]],
            ],
            Self::L => [
                [[0, 0, 0, 0], [0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0]],
                [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
            ],
            Self::O => [
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
            ],
            Self::S => [
                [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0]],
                [[0, 0, 0, 0], [1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]],
            ],
            Self::T => [
                [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0]],
            ],
            Self::Z => [
                [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                [[0, 0, 0, 0], [0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0]],
                [[0, 0, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0]],
            ],
        };

        dir.map(|dir| dir.map(|row| row.map(|v| v != 0)))
    }

    pub const fn wall_kicks(&self, direction: &Direction, diff: &DirectionDiff) -> [(i8, i8); 5] {
        match self {
            Self::J | Self::L | Self::S | Self::T | Self::Z => match (direction, diff) {
                (Direction::Up, DirectionDiff::Cw) => [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (Direction::Up, DirectionDiff::Ccw) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],

                (Direction::Right, DirectionDiff::Cw) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (Direction::Right, DirectionDiff::Ccw) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],

                (Direction::Down, DirectionDiff::Cw) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                (Direction::Down, DirectionDiff::Ccw) => {
                    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
                }

                (Direction::Left, DirectionDiff::Cw) => {
                    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
                }
                (Direction::Left, DirectionDiff::Ccw) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            },
            Self::I => match (direction, diff) {
                (Direction::Up, DirectionDiff::Cw) => [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (Direction::Up, DirectionDiff::Ccw) => [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                (Direction::Right, DirectionDiff::Cw) => [(0, 0), (-1, 0), (2, 0), (1, 2), (2, -1)],
                (Direction::Right, DirectionDiff::Ccw) => {
                    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]
                }
                (Direction::Down, DirectionDiff::Cw) => [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (Direction::Down, DirectionDiff::Ccw) => {
                    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]
                }
                (Direction::Left, DirectionDiff::Cw) => [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (Direction::Left, DirectionDiff::Ccw) => {
                    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]
                }
            },
            Self::O => [(0, 0); 5],
        }
    }
}
