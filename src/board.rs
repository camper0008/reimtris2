use std::ops::{Deref, DerefMut};

use crate::{CurrentTetromino, Tetromino};

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

    pub fn colliding(
        &self,
        CurrentTetromino {
            tetromino,
            direction,
            x: cur_x,
            y: cur_y,
        }: &CurrentTetromino,
    ) -> bool {
        let pattern = tetromino.direction_pattern(direction);

        for (y, row) in pattern.iter().enumerate() {
            for x in row
                .iter()
                .enumerate()
                .filter(|(_, exists)| **exists)
                .map(|(x, _)| x)
            {
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

                if self.0[y as usize][x as usize].is_some() {
                    return true;
                }
            }
        }

        false
    }
}
