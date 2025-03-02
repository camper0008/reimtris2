#[derive(Debug, PartialEq)]
enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Copy, Clone)]
struct Rgb(u8, u8, u8);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum DirectionDiff {
    CW,
    CCW,
}

impl Tetromino {
    fn new_random() -> Self {
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

    fn direction_pattern(&self, direction: Direction) -> [[bool; 4]; 4] {
        let idx = match direction {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        };

        self.directions()[idx]
    }

    fn rotation_pattern_from_string(pattern: &'static str) -> [[bool; 4]; 4] {
        pattern
            .split_whitespace()
            .map(|row| {
                let row: [char; 4] = row.chars().collect::<Vec<_>>().try_into().unwrap();
                row.map(|c| {
                    if c == '#' {
                        true
                    } else if c == '-' {
                        false
                    } else {
                        panic!("invalid rotation pattern");
                    }
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    const fn directions(&self) -> [[[bool; 4]; 4]; 4] {
        match self {
            Self::I => [
                Self::rotation_pattern_from_string(
                    r"
                ----
                ####
                ----
                ----
                ",
                ),
                Self::rotation_pattern_from_string(
                    r"
                --#-
                --#-
                --#-
                --#-
                ",
                ),
                Self::rotation_pattern_from_string(
                    r"
                ----
                ----
                ####
                ----
                ",
                ),
                Self::rotation_pattern_from_string(
                    r"
                -#--
                -#--
                -#--
                -#--
                ",
                ),
            ],
            Self::J => [
                Self::rotation_pattern_from_string(
                    r"
                ----
                #---
                ###-
                ----
                ",
                ),
                Self::rotation_pattern_from_string(
                    r"
                ----
                -##-
                -#--
                -#--
                ",
                ),
                Self::rotation_pattern_from_string(
                    r"
                ----
                ----
                ###-
                --#-
                ",
                ),
                Self::rotation_pattern_from_string(
                    r"
                ----
                -#--
                -#--
                ##--
                ",
                ),
            ],
            Self::L => [
                Self::rotation_pattern_from_string(
                    r"
                ----
                --#-
                ###-
                ----
                ",
                ),
                Self::rotation_pattern_from_string(
                    r"
                ----
                -#--
                -#--
                -##-
                ",
                ),
                Self::rotation_pattern_from_string(
                    r"
                ----
                ----
                ###-
                #---
                ",
                ),
                Self::rotation_pattern_from_string(
                    r"
                ----
                ##--
                -#--
                -#--
                ",
                ),
            ],
            Self::O => {
                [Self::rotation_pattern_from_string(
                    r"
                ----
                -##-
                -##-
                ----
                ",
                ); 4]
            }
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
        }
    }

    const fn wallkicks(&self, direction: Direction, diff: DirectionDiff) -> [(i8, i8); 5] {
        match self {
            Self::J | Self::L | Self::S | Self::T | Self::Z => match (direction, diff) {
                (Direction::Up, DirectionDiff::CW) => [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (Direction::Up, DirectionDiff::CCW) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],

                (Direction::Right, DirectionDiff::CW) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (Direction::Right, DirectionDiff::CCW) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],

                (Direction::Down, DirectionDiff::CW) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                (Direction::Down, DirectionDiff::CCW) => {
                    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
                }

                (Direction::Left, DirectionDiff::CW) => {
                    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
                }
                (Direction::Left, DirectionDiff::CCW) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            },
            Self::I => match (direction, diff) {
                (Direction::Up, DirectionDiff::CW) => [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (Direction::Up, DirectionDiff::CCW) => [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                (Direction::Right, DirectionDiff::CW) => [(0, 0), (-1, 0), (2, 0), (1, 2), (2, -1)],
                (Direction::Right, DirectionDiff::CCW) => {
                    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]
                }
                (Direction::Down, DirectionDiff::CW) => [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (Direction::Down, DirectionDiff::CCW) => {
                    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]
                }
                (Direction::Left, DirectionDiff::CW) => [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (Direction::Left, DirectionDiff::CCW) => {
                    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]
                }
            },
            Self::O => [(0, 0); 5],
        }
    }
}

struct CurrentTetromino {
    tetromino: Tetromino,
    direction: Direction,
    x: i8,
    y: i8,
}

impl CurrentTetromino {
    fn new(tetromino: Tetromino) -> Self {
        const PIECE_WIDTH: i8 = 2;
        Self {
            tetromino,
            direction: Direction::Up,
            x: (Board::WIDTH as i8 - PIECE_WIDTH) / 2,
            y: -1,
        }
    }
}

struct Board([[Rgb; Self::WIDTH]; Self::HEIGHT]);

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
}

impl Board {
    pub fn new() -> Self {
        Board([[Rgb(0, 0, 0); Self::WIDTH]; Self::HEIGHT])
    }

    pub fn colliding(
        &self,
        CurrentTetromino {
            tetromino,
            direction,
            x: cur_x,
            y: cur_y,
        }: CurrentTetromino,
    ) -> bool {
        let pattern = tetromino.direction_pattern(direction);

        for y in 0..pattern.len() {
            for x in 0..pattern[y].len() {
                if pattern[y][x] == 0 {
                    continue;
                }

                let x = x as i8 + cur_x;
                let y = y as i8 + cur_y;

                if y < 0 {
                    continue;
                }

                if y >= Board::HEIGHT as i8 {
                    return true;
                }

                if x < 0 || x >= Board::WIDTH as i8 {
                    return true;
                }

                if self.0[y as usize][x as usize] != Rgb(0, 0, 0) {
                    return true;
                }
            }
        }

        false
    }
}

struct Game {
    board: Board,
    next_tetrominos: [Tetromino; 3],
    current_tetromino: CurrentTetromino,
    held_tetromino: Option<Tetromino>,
    has_swapped_held: bool,
    score: Score,
}

struct Score {
    level: usize,
    score: usize,
    lines: usize,
    combo: usize,
}

impl Score {
    const fn new() -> Self {
        Self {
            level: 0,
            score: 0,
            lines: 0,
            combo: 0,
        }
    }
}

impl Game {
    fn take_next_in_bag(&mut self, mut last: Tetromino) -> Tetromino {
        for value in self.next_tetrominos.iter_mut().rev() {
            std::mem::swap(value, &mut last)
        }
        last
    }

    fn try_swap_tetromino(&mut self) {
        if self.has_swapped_held {
            return;
        }
        self.has_swapped_held = true;
        let held_or_first_in_bag_tetromino = self
            .held_tetromino
            .take()
            .unwrap_or_else(|| self.take_next_in_bag(Tetromino::new_random()));
        let current_tetromino = CurrentTetromino::new(held_or_first_in_bag_tetromino);
        let old_tetromino = std::mem::replace(&mut self.current_tetromino, current_tetromino);
        self.held_tetromino.replace(old_tetromino.tetromino);
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::{Board, CurrentTetromino, Game, Score, Tetromino};

    #[test]
    fn advance_bag() {
        let mut game = Game {
            board: Board::new(),
            score: Score::new(),
            next_tetrominos: [Tetromino::I, Tetromino::J, Tetromino::O],
            current_tetromino: CurrentTetromino::new(Tetromino::J),
            held_tetromino: None,
            has_swapped_held: false,
        };
        assert_eq!(game.take_next_in_bag(Tetromino::S), Tetromino::I);
        assert_eq!(
            game.next_tetrominos,
            [Tetromino::J, Tetromino::O, Tetromino::S]
        );
    }
}
