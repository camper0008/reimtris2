use crate::{
    board::Board,
    game::{CurrentTetromino, Score},
    tetromino::{Direction, Tetromino},
};

pub trait UiCtx<Err> {
    fn window_size(&self) -> Result<(i32, i32), Err>;
    fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, rgb: &Rgb) -> Result<(), Err>;
    fn outline_rect(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        rgb: &Rgb,
    ) -> Result<(), Err>;
    fn text_size<P: AsRef<std::path::Path>, Text: AsRef<str>>(
        &mut self,
        font: P,
        text: Text,
    ) -> Result<(i32, i32), Err>;
    fn fill_text<P: AsRef<std::path::Path>, Text: AsRef<str>>(
        &mut self,
        font: P,
        text: Text,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), Err>;
    fn clear(&mut self, rgb: &Rgb) -> Result<(), Err>;
}

pub trait GameUiCtx<Err>: UiCtx<Err> {
    fn tile_size(&self) -> i32 {
        24
    }

    fn draw_tetromino_from_parts(
        &mut self,
        x: i8,
        y: i8,
        color: Rgb,
        pattern: &Vec<(usize, usize)>,
        filled: bool,
    ) -> Result<(), Err> {
        for (x_offset, y_offset) in pattern {
            let x = *x_offset as i8 + x;
            let y = *y_offset as i8 + y;

            if y < 0 {
                continue;
            }

            self.draw_board_tile(x as i32, y as i32, &color, filled)?
        }
        Ok(())
    }

    fn draw_board_tile(&mut self, x: i32, y: i32, color: &Rgb, filled: bool) -> Result<(), Err> {
        let (win_width, win_height) = self.window_size()?;
        let x = center(self.tile_size() * Board::WIDTH as i32, win_width) + x * self.tile_size();
        let y = center(self.tile_size() * Board::HEIGHT as i32, win_height) + y * self.tile_size();
        if filled {
            self.fill_rect(x, y, self.tile_size(), self.tile_size(), color)?;
        } else {
            self.outline_rect(x, y, self.tile_size(), self.tile_size(), color)?;
        }
        Ok(())
    }

    fn draw_centered_tetromino(
        &mut self,
        tetromino: &Tetromino,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), Err> {
        let color = Rgb::from_tetromino(tetromino);
        let pattern = tetromino.pattern(&Direction::Up);

        let min_x_offset = pattern
            .iter()
            .min_by(|left, right| left.0.cmp(&right.0))
            .expect("pattern's len > 0")
            .0;
        let min_y_offset = pattern
            .iter()
            .min_by(|left, right| left.1.cmp(&right.1))
            .expect("pattern's len > 0")
            .1;

        let (x_len, y_len) = {
            let max_x_offset = pattern
                .iter()
                .max_by(|left, right| left.0.cmp(&right.0))
                .expect("pattern's len > 0")
                .0;

            let max_y_offset = pattern
                .iter()
                .max_by(|left, right| left.1.cmp(&right.1))
                .expect("pattern's len > 0")
                .1;

            (
                1 + max_x_offset - min_x_offset,
                1 + max_y_offset - min_y_offset,
            )
        };

        let x = x + center(self.tile_size() * x_len as i32, width);
        let y = y + center(self.tile_size() * y_len as i32, height);

        for (x_offset, y_offset) in pattern {
            let x_offset = (x_offset - min_x_offset) as i32;
            let y_offset = (y_offset - min_y_offset) as i32;
            let x = x + (x_offset * self.tile_size());
            let y = y + (y_offset * self.tile_size());
            self.fill_rect(x, y, self.tile_size(), self.tile_size(), &color)?;
        }

        Ok(())
    }

    fn draw_held_tetromino(
        &mut self,
        held: &Option<Tetromino>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), Err> {
        self.fill_rect(x, y, width, height, &Rgb(0, 0, 0))?;
        self.outline_rect(x - 1, y - 1, width + 2, height + 2, &Rgb(255, 255, 255))?;

        let Some(tetromino) = held else {
            return Ok(());
        };
        self.draw_centered_tetromino(&tetromino, x, y, width, height)?;

        Ok(())
    }

    fn draw_next_up(
        &mut self,
        next_up: &[Tetromino; 3],
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), Err> {
        self.fill_rect(x, y, width, height * 3, &Rgb(0, 0, 0))?;
        self.outline_rect(x - 1, y - 1, width + 2, height * 3 + 2, &Rgb(255, 255, 255))?;

        for (offset, tetromino) in next_up.iter().enumerate() {
            self.draw_centered_tetromino(tetromino, x, y + offset as i32 * height, width, height)?;
        }

        Ok(())
    }

    fn draw_bag(&mut self, held: &Option<Tetromino>, next_up: &[Tetromino; 3]) -> Result<(), Err> {
        let (win_width, win_height) = self.window_size()?;
        let x = center(self.tile_size() * Board::WIDTH as i32, win_width);
        let y = center(self.tile_size() * Board::HEIGHT as i32, win_height);

        let width = self.tile_size() * 5;
        let height = self.tile_size() * 4;
        let x = x - width - self.tile_size();

        self.draw_held_tetromino(&held, x, y, width, height)?;

        let y = y + height + self.tile_size();

        self.draw_next_up(next_up, x, y, width, height)?;

        Ok(())
    }

    fn draw_score<P: AsRef<std::path::Path>>(&mut self, font: P, score: &Score) -> Result<(), Err> {
        let (win_width, win_height) = self.window_size()?;
        let board_width = self.tile_size() * Board::WIDTH as i32;
        let board_height = self.tile_size() * Board::HEIGHT as i32;
        let x = center(board_width, win_width) + board_width + self.tile_size();
        let y = center(board_height, win_height) + self.tile_size();

        let level = format!("level: {}", score.level);
        let lines = format!("lines: {}", score.lines);
        let points = format!("points: {}", score.points);

        let level_size = self.text_size(font.as_ref(), &level)?;
        let lines_size = self.text_size(font.as_ref(), &lines)?;
        let points_size = self.text_size(font.as_ref(), &points)?;

        self.fill_text(font.as_ref(), level, x, y, level_size.0, level_size.1)?;
        let y = y + level_size.1 + self.tile_size();
        self.fill_text(font.as_ref(), lines, x, y, lines_size.0, lines_size.1)?;
        let y = y + lines_size.1 + self.tile_size();
        self.fill_text(font.as_ref(), points, x, y, points_size.0, points_size.1)?;

        Ok(())
    }

    fn draw_board(&mut self, board: &Board, current: &CurrentTetromino) -> Result<(), Err> {
        let (win_width, win_height) = self.window_size()?;
        self.outline_rect(
            center(self.tile_size() * Board::WIDTH as i32, win_width) - 1,
            center(self.tile_size() * Board::HEIGHT as i32, win_height) - 1,
            self.tile_size() * Board::WIDTH as i32 + 2,
            self.tile_size() * Board::HEIGHT as i32 + 2,
            &Rgb(255, 255, 255),
        )?;

        for (y, row) in board.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                let color = match piece {
                    Some(t) => Rgb::from_tetromino(t),
                    None => Rgb(0, 0, 0),
                };
                self.draw_board_tile(x as i32, y as i32, &color, true)?
            }
        }

        let pattern = current.tetromino.pattern(&current.direction);

        self.draw_tetromino_from_parts(
            current.x,
            board.lowest_y(&current),
            Rgb(255, 255, 255),
            &pattern,
            false,
        )?;

        self.draw_tetromino_from_parts(
            current.x,
            current.y,
            Rgb::from_tetromino(&current.tetromino),
            &pattern,
            true,
        )?;

        Ok(())
    }

    fn draw_important_text<P: AsRef<std::path::Path>, Text: AsRef<str>>(
        &mut self,
        font: P,
        text: Text,
    ) -> Result<(), Err> {
        let (win_width, win_height) = self.window_size()?;
        let size = self.text_size(font.as_ref(), text.as_ref())?;
        let width = size.0;
        let height = size.1;

        let x = center(width, win_width);
        let y = center(height, win_height);

        let padding = 8;

        self.outline_rect(
            x - padding - 1,
            y - padding - 1,
            width + padding * 2 + 2,
            height + padding * 2 + 2,
            &Rgb(255, 255, 255),
        )?;

        self.fill_rect(
            x - padding,
            y - padding,
            width + padding * 2,
            height + padding * 2,
            &Rgb(16, 16, 16),
        )?;
        self.fill_text(font, text, x, y, width, height)?;

        Ok(())
    }
}

impl<T, Err> GameUiCtx<Err> for T where T: UiCtx<Err> {}

pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn from_tetromino(tetromino: &Tetromino) -> Self {
        match tetromino {
            Tetromino::I => Self(0, 255, 255),
            Tetromino::J => Self(0, 0, 255),
            Tetromino::L => Self(255, 128, 0),
            Tetromino::O => Self(255, 255, 0),
            Tetromino::S => Self(0, 255, 0),
            Tetromino::T => Self(255, 0, 255),
            Tetromino::Z => Self(255, 0, 0),
        }
    }
}

fn center(length: i32, max: i32) -> i32 {
    (max - length) / 2
}
