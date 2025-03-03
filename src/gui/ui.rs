use crate::{
    board::Board,
    game::CurrentTetromino,
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
        let tile_size = 24;
        let (win_width, win_height) = self.window_size()?;
        let x = center(tile_size * Board::WIDTH as i32, win_width) + x * tile_size;
        let y = center(tile_size * Board::HEIGHT as i32, win_height) + y * tile_size;
        if filled {
            self.fill_rect(x, y, tile_size, tile_size, color)?;
        } else {
            self.outline_rect(x, y, tile_size, tile_size, color)?;
        }
        Ok(())
    }

    fn draw_bag(&mut self, held: &Option<Tetromino>, next_up: &[Tetromino; 3]) -> Result<(), Err> {
        let (win_width, win_height) = self.window_size()?;
        let x = center(24 * Board::WIDTH as i32, win_width);
        let y = center(24 * Board::HEIGHT as i32, win_height);

        let size = 24 * 5;
        let x = x - size - 16;

        self.fill_rect(x, y, size, size, &Rgb(0, 0, 0))?;
        self.outline_rect(x - 1, y - 1, size + 2, size + 2, &Rgb(255, 255, 255))?;

        if let Some(tetromino) = held {
            let color = Rgb::from_tetromino(&tetromino);
            let pattern = tetromino.pattern(&Direction::Up);
            for (x_offset, y_offset) in pattern {
                let x = x + (x_offset * 24) as i32;
                let y = y + (y_offset * 24) as i32;
                self.fill_rect(x, y, 24, 24, &color)?;
            }
        }

        Ok(())
    }

    fn draw_board(&mut self, board: &Board, current: &CurrentTetromino) -> Result<(), Err> {
        let (win_width, win_height) = self.window_size()?;
        self.outline_rect(
            center(24 * Board::WIDTH as i32, win_width) - 1,
            center(24 * Board::HEIGHT as i32, win_height) - 1,
            24 * Board::WIDTH as i32 + 2,
            24 * Board::HEIGHT as i32 + 2,
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
