use crate::actions::{Action, ActionsHeld};
use crate::board::Board;
use crate::tetromino::{Direction, DirectionDiff, Tetromino};

pub struct CurrentTetromino {
    pub tetromino: Tetromino,
    pub direction: Direction,
    pub x: i8,
    pub y: i8,
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

pub struct Game {
    pub game_over: bool,
    pub board: Board,
    pub next_tetrominos: [Tetromino; 3],
    pub current_tetromino: CurrentTetromino,
    pub held_tetromino: Option<Tetromino>,
    has_swapped_held: bool,
    pub score: Score,
    pub ticks: usize,
}

pub enum SoundEffect {
    HardDrop,
    LineClear(usize),
    Move,
    Rotation,
}

pub struct Score {
    pub level: usize,
    pub points: usize,
    pub lines: usize,
    pub combo: usize,
    back_to_back: bool,
}

impl Score {
    const fn new() -> Self {
        Self {
            level: 0,
            points: 0,
            lines: 0,
            combo: 0,
            back_to_back: false,
        }
    }

    fn level_up(&mut self, lines_cleared: usize) {
        self.lines += lines_cleared;
        if self.lines > self.level * 5 {
            self.level += 1;
            self.lines = 0;
        }
    }

    fn point_multiplier_from_lines_cleared(lines_cleared: usize) -> f32 {
        match lines_cleared {
            0 => 0.0,
            1 => 100.0,
            2 => 300.0,
            3 => 500.0,
            4 => 800.0,
            _ => unreachable!("we cannot clear more than 4 lines"),
        }
    }

    fn combos(&self, lines_cleared: usize) -> usize {
        if lines_cleared > 0 {
            self.combo * 50 * self.level
        } else {
            0
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_over: false,
            board: Board::new(),
            next_tetrominos: std::array::from_fn(|_| Tetromino::random()),
            current_tetromino: CurrentTetromino::new(Tetromino::random()),
            held_tetromino: None,
            has_swapped_held: false,
            score: Score::new(),
            ticks: 0,
        }
    }
    fn take_next_in_bag(&mut self, mut last: Tetromino) -> Tetromino {
        for value in self.next_tetrominos.iter_mut().rev() {
            std::mem::swap(value, &mut last)
        }
        last
    }

    fn try_hard_drop(&mut self, actions: &ActionsHeld, effects: &mut Vec<SoundEffect>) {
        if !actions.just_pressed(self.ticks, &Action::HardDrop) {
            return;
        }
        let start_y = self.current_tetromino.y;
        loop {
            self.current_tetromino.y += 1;
            if !self.board.colliding(&self.current_tetromino) {
                continue;
            }
            self.current_tetromino.y -= 1;
            self.score.points += (self.current_tetromino.y - start_y) as usize * 2;
            self.place_current_tetromino();
            self.check_line_clears(effects);
            break;
        }
    }

    fn soft_drop(&mut self, actions: &ActionsHeld, effects: &mut Vec<SoundEffect>) {
        let mut delay = 32 - self.score.level * 2;
        if actions.contains_key(&Action::SoftDrop) {
            delay /= 10;
        }

        if self.ticks % delay != 0 {
            return;
        }

        self.current_tetromino.y += 1;
        if self.board.colliding(&self.current_tetromino) {
            self.current_tetromino.y -= 1;
            self.place_current_tetromino();
            self.check_line_clears(effects);
        } else if actions.contains_key(&Action::SoftDrop) {
            self.score.points += 1;
        }
    }

    fn try_move_horizontally(&mut self, actions: &ActionsHeld, effects: &mut Vec<SoundEffect>) {
        for key in [Action::Left, Action::Right] {
            let just_pressed = actions.just_pressed(self.ticks, &key);
            let long_press = actions.held_for(self.ticks, &key, |held_for| held_for > 15);
            if !just_pressed && !long_press {
                continue;
            }
            let offset = match key {
                Action::Left => -1,
                Action::Right => 1,
                _ => unreachable!(),
            };
            self.current_tetromino.x += offset;
            if self.board.colliding(&self.current_tetromino) {
                self.current_tetromino.x -= offset;
            } else {
                effects.push(SoundEffect::Move);
            }
        }
    }

    fn check_line_clears(&mut self, effects: &mut Vec<SoundEffect>) {
        let lines_cleared = self.board.lines_cleared();

        self.score.level_up(lines_cleared);

        let mut points =
            self.score.level as f32 * Score::point_multiplier_from_lines_cleared(lines_cleared);

        if self.score.back_to_back && lines_cleared == 4 {
            points *= 1.5;
        }
        points += self.score.combos(lines_cleared) as f32;

        self.score.points += points as usize;

        if lines_cleared == 4 {
            self.score.back_to_back = true;
        } else if lines_cleared > 0 {
            self.score.back_to_back = false;
        }

        if lines_cleared > 0 {
            self.score.combo += 1;
            effects.push(SoundEffect::LineClear(lines_cleared));
        } else {
            self.score.combo = 0;
            effects.push(SoundEffect::HardDrop);
        }
    }

    pub fn step(&mut self, actions: &ActionsHeld) -> Vec<SoundEffect> {
        if self.game_over {
            panic!("should check if game is over before stepping");
        }
        let mut effects = Vec::new();
        self.try_hard_drop(actions, &mut effects);
        self.soft_drop(actions, &mut effects);
        self.try_move_horizontally(actions, &mut effects);

        if actions.just_pressed(self.ticks, &Action::Swap) {
            self.try_swap_tetromino(&mut effects);
        }

        for (control, direction) in [
            (Action::RotateCw, DirectionDiff::Cw),
            (Action::RotateCcw, DirectionDiff::Ccw),
        ] {
            if !actions.just_pressed(self.ticks, &control) {
                continue;
            }
            self.try_rotate(direction, &mut effects);
        }
        self.ticks += 1;
        effects
    }

    fn try_rotate(&mut self, diff: DirectionDiff, effects: &mut Vec<SoundEffect>) {
        let rotated = self.current_tetromino.direction.rotate(&diff);
        let old_direction = std::mem::replace(&mut self.current_tetromino.direction, rotated);
        if !self.board.colliding(&self.current_tetromino) {
            effects.push(SoundEffect::Rotation);
            return;
        }
        let wall_kicks = self
            .current_tetromino
            .tetromino
            .wall_kicks(&old_direction, &diff);

        for (x, y) in wall_kicks {
            self.current_tetromino.x += x;
            self.current_tetromino.y += y;
            if !(self.board.colliding(&self.current_tetromino)) {
                effects.push(SoundEffect::Rotation);
                return;
            }
            self.current_tetromino.x -= x;
            self.current_tetromino.y -= y;
        }

        self.current_tetromino.direction = old_direction;
    }

    fn place_current_tetromino(&mut self) {
        let next = CurrentTetromino::new(self.take_next_in_bag(Tetromino::random()));
        let current = std::mem::replace(&mut self.current_tetromino, next);
        let pattern = current.tetromino.direction_pattern(&current.direction);

        if current.y <= 0 {
            self.game_over = true;
        }

        for (y, row) in pattern.iter().enumerate() {
            for x in row
                .iter()
                .enumerate()
                .filter(|(_, exists)| **exists)
                .map(|(x, _)| x)
            {
                let y = current.y + y as i8;
                if y < 0 {
                    continue;
                }
                let y = y as usize;
                let x = (current.x + x as i8) as usize;
                self.board[y][x] = Some(current.tetromino.clone());
            }
        }

        self.has_swapped_held = false;
    }

    fn try_swap_tetromino(&mut self, effects: &mut Vec<SoundEffect>) {
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
        effects.push(SoundEffect::Rotation);
    }
}

#[cfg(test)]
mod test {
    use super::{Board, CurrentTetromino, Game, Score, Tetromino};

    #[test]
    fn advance_bag() {
        let mut game = Game {
            game_over: false,
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
