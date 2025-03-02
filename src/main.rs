#![allow(dead_code)]

use actions::{Controls, ControlsHeld};
use board::Board;
use tetromino::{Direction, DirectionDiff, Tetromino};

mod actions;
mod board;
mod tetromino;

struct Rgb(u8, u8, u8);

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

struct Game {
    board: Board,
    next_tetrominos: [Tetromino; 3],
    current_tetromino: CurrentTetromino,
    held_tetromino: Option<Tetromino>,
    has_swapped_held: bool,
    score: Score,
    ticks: usize,
}

struct Score {
    level: usize,
    points: usize,
    lines: usize,
    combo: usize,
}

impl Score {
    const fn new() -> Self {
        Self {
            level: 0,
            points: 0,
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

    fn hard_drop(&mut self, controls: &ControlsHeld) {
        if controls.contains_key(&Controls::HardDrop) {
            loop {
                todo!()
            }
        }
    }

    fn soft_drop(&mut self, controls: &ControlsHeld) {
        let mut delay = 32 - self.score.level * 2;
        if controls.contains_key(&Controls::SoftDrop) {
            delay /= 10;
        }

        if self.ticks % delay != 0 {
            return;
        }

        self.current_tetromino.y += 1;
        if self.board.colliding(&self.current_tetromino) {
            self.current_tetromino.y -= 1;
            self.place_current_tetromino();
            self.check_line_clears();
            self.has_swapped_held = false;
        } else if controls.contains_key(&Controls::SoftDrop) {
            self.score.points += 1;
        }
    }

    fn move_horizontally(&mut self, controls: &ControlsHeld) {
        for key in [Controls::Left, Controls::Right] {
            let Some(held_since) = controls.get(&key) else {
                continue;
            };
            let held_for = self.ticks - held_since;
            if held_for < 15 {
                continue;
            }
            let offset = match key {
                Controls::Left => -1,
                Controls::Right => 1,
                _ => unreachable!(),
            };
            self.current_tetromino.x += offset;
            if self.board.colliding(&self.current_tetromino) {
                self.current_tetromino.x -= offset;
            }
        }
    }

    fn check_line_clears(&self) {
        todo!()
    }

    fn step(&mut self, controls: &ControlsHeld) {
        // TODO: ensure game is running at 60fps (`if !check_update_time(context, 60) { return; }`)
        self.ticks += 1;
        self.soft_drop(controls);
        self.move_horizontally(controls);
    }

    fn try_rotate(&mut self, diff: DirectionDiff) -> bool {
        let rotated = self.current_tetromino.direction.rotate(&diff);
        let old_direction = std::mem::replace(&mut self.current_tetromino.direction, rotated);
        if !self.board.colliding(&self.current_tetromino) {
            return true;
        }
        let wall_kicks = self
            .current_tetromino
            .tetromino
            .wall_kicks(&old_direction, &diff);

        for (x, y) in wall_kicks {
            self.current_tetromino.x += x;
            self.current_tetromino.y += y;
            if !(self.board.colliding(&self.current_tetromino)) {
                return true;
            }
            self.current_tetromino.x -= x;
            self.current_tetromino.y -= y;
        }

        self.current_tetromino.direction = old_direction;
        false
    }

    fn place_current_tetromino(&mut self) {
        let next = CurrentTetromino::new(self.take_next_in_bag(Tetromino::random()));
        let current = std::mem::replace(&mut self.current_tetromino, next);
        let pattern = current.tetromino.direction_pattern(&current.direction);

        for (y, row) in pattern.iter().enumerate() {
            for x in row
                .iter()
                .enumerate()
                .filter(|(_, exists)| **exists)
                .map(|(x, _)| x)
            {
                let y = (current.y + y as i8) as usize;
                let x = (current.x + x as i8) as usize;
                self.board[y][x] = Some(current.tetromino.clone());
            }
        }
    }

    fn try_swap_tetromino(&mut self) {
        if self.has_swapped_held {
            return;
        }
        self.has_swapped_held = true;
        let held_or_first_in_bag_tetromino = self
            .held_tetromino
            .take()
            .unwrap_or_else(|| self.take_next_in_bag(Tetromino::random()));
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
            ticks: 0,
        };
        assert_eq!(game.take_next_in_bag(Tetromino::S), Tetromino::I);
        assert_eq!(
            game.next_tetrominos,
            [Tetromino::J, Tetromino::O, Tetromino::S]
        );
    }
}
