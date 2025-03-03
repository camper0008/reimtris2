use tetromino::Tetromino;

mod actions;
mod board;
mod game;
mod gui;
mod tetromino;

fn main() {
    gui::start_game().unwrap();
}
