use std::ops::{Deref, DerefMut};

use crate::{game::CurrentTetromino, Tetromino};

#[derive(PartialEq)]
pub struct Board([[Option<Tetromino>; Self::WIDTH]; Self::HEIGHT]);

impl Deref for Board {
    type Target = [[Option<Tetromino>; Self::WIDTH]; Self::HEIGHT];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Board {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 20;

    pub fn new() -> Self {
        let board = std::array::from_fn(|_| std::array::from_fn(|_| None));
        Board(board)
    }

    pub fn lowest_y(
        &self,
        CurrentTetromino {
            tetromino,
            direction,
            x,
            y,
        }: &CurrentTetromino,
    ) -> i8 {
        let pattern = tetromino.pattern(direction);
        let mut y = *y;
        loop {
            if self.pattern_and_position_colliding(&pattern, *x, y) {
                break y - 1;
            }
            y += 1;
        }
    }

    fn pattern_and_position_colliding(&self, pattern: &Vec<(usize, usize)>, x: i8, y: i8) -> bool {
        for (x_offset, y_offset) in pattern {
            let x = *x_offset as i8 + x;
            let y = *y_offset as i8 + y;

            if y < 0 {
                continue;
            }

            if y >= Board::HEIGHT as i8 {
                return true;
            }

            if x < 0 || x >= Board::WIDTH as i8 {
                return true;
            }

            if self.0[y as usize][x as usize].is_some() {
                return true;
            }
        }

        false
    }

    pub fn colliding(
        &self,
        CurrentTetromino {
            tetromino,
            direction,
            x,
            y,
        }: &CurrentTetromino,
    ) -> bool {
        self.pattern_and_position_colliding(&tetromino.pattern(direction), *x, *y)
    }

    pub fn lines_cleared(&mut self) -> usize {
        let line_clears: Vec<_> = self
            .iter()
            .enumerate()
            .filter_map(|(i, row)| if !row.contains(&None) { Some(i) } else { None })
            .collect();

        let mut lines_cleared = 0;
        for i in (0..self.len()).rev() {
            let blank_line = std::array::from_fn(|_| None);
            let line = std::mem::replace(&mut self[i], blank_line);
            self[i + lines_cleared] = line;

            if line_clears.contains(&i) {
                lines_cleared += 1;
            }
        }
        lines_cleared
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| match t {
                        Some(t) => match t {
                            Tetromino::I => "I",
                            Tetromino::J => "J",
                            Tetromino::L => "L",
                            Tetromino::O => "O",
                            Tetromino::S => "S",
                            Tetromino::T => "T",
                            Tetromino::Z => "Z",
                        },
                        None => ".",
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{t}")
    }
}

#[cfg(test)]
mod test {
    use super::Board;

    fn board_from_str(str: &'static str) -> Board {
        use crate::Tetromino::*;
        Board(
            str.split_whitespace()
                .map(|row| {
                    let row: [char; Board::WIDTH] = row
                        .chars()
                        .collect::<Vec<_>>()
                        .try_into()
                        .expect("invalid board row");
                    row.map(|char| match char {
                        '.' => None,
                        'I' => Some(I),
                        'J' => Some(J),
                        'L' => Some(L),
                        'O' => Some(O),
                        'S' => Some(S),
                        'T' => Some(T),
                        'Z' => Some(Z),
                        c => panic!("invalid board char '{c}'"),
                    })
                })
                .collect::<Vec<_>>()
                .try_into()
                .expect("invalid board content"),
        )
    }

    #[test]
    fn line_clear() {
        let mut board = board_from_str(
            "
            ..........
            ..........
            ..........
            ..........
            ..OOOOOOOO
            OOOOOOOO..
            JJJJJJJJJJ
            JJJJJJJJJJ
            ..JJJJJJJJ
            JJJJJJJJ..
            JJJJJJJJ..
            JJJJJJJJ..
            JJJJJJJJJJ
            JJJJJJJJJJ
            JJJJJJJJJJ
            JJJJJJJJJJ
            JJJJJJJJJJ
            JJJJJJJJJJ
            JJJJJJJJJJ
            JJJJJJJJJJ
        ",
        );

        let after = board_from_str(
            "
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..........
            ..OOOOOOOO
            OOOOOOOO..
            ..JJJJJJJJ
            JJJJJJJJ..
            JJJJJJJJ..
            JJJJJJJJ..
        ",
        );

        assert_eq!(board.lines_cleared(), 10);

        assert_eq!(board, after);
    }
}
