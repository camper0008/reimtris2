use config::Config;
use tetromino::Tetromino;

mod actions;
mod board;
mod config;
mod game;
mod gui;
mod tetromino;

fn main() -> Result<(), String> {
    let config = {
        let base = xdg::BaseDirectories::new().map_err(|err| err.to_string())?;
        let path = base
            .place_config_file("reimtris2/config.toml")
            .map_err(|err| err.to_string())?;
        let config = Config::from_file(path)?;
        config
    };
    gui::start_game(config)
}
