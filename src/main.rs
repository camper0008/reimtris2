use tetromino::Tetromino;

mod actions;
mod board;
mod game;
mod sdl_impl;
mod tetromino;

struct Rgb(u8, u8, u8);

impl Rgb {
    fn from_tetromino(tetromino: &Tetromino) -> Self {
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

fn main() {
    sdl_impl::start_game();
}
