use crate::actions::{Action, ActionsHeld};
use crate::game::Game;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;

use super::audio::{self};
use super::ui::{GameUiCtx, Rgb, UiCtx};

fn font_texture<'font, 'a, P: AsRef<std::path::Path>, Text: AsRef<str>, C>(
    font: P,
    text: Text,
    ttf_context: &'a Sdl2TtfContext,
    texture_creator: &'font TextureCreator<C>,
) -> Result<Texture<'font>, String> {
    let font = ttf_context.load_font(font, 24)?;
    let game_over_text = font
        .render(text.as_ref())
        .solid(Color::RGB(255, 255, 255))
        .map_err(|err| err.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(game_over_text)
        .map_err(|err| err.to_string())?;

    Ok(texture)
}

struct SdlUiCtx {
    canvas: WindowCanvas,
    ttf: Sdl2TtfContext,
}

impl SdlUiCtx {
    fn present(&mut self) {
        self.canvas.present();
    }
}

impl UiCtx<String> for SdlUiCtx {
    fn window_size(&self) -> Result<(i32, i32), String> {
        let (width, height) = self.canvas.window().size();
        Ok((width as i32, height as i32))
    }

    fn fill_rect(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        rgb: &super::ui::Rgb,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(rgb.0, rgb.1, rgb.2));
        self.canvas
            .fill_rect(Rect::new(x, y, width as u32, height as u32))?;
        Ok(())
    }

    fn outline_rect(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        rgb: &super::ui::Rgb,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(rgb.0, rgb.1, rgb.2));
        self.canvas
            .draw_rect(Rect::new(x, y, width as u32, height as u32))?;
        Ok(())
    }

    fn text_size<P: AsRef<std::path::Path>, Text: AsRef<str>>(
        &mut self,
        font: P,
        text: Text,
    ) -> Result<(i32, i32), String> {
        let texture_creator = self.canvas.texture_creator();
        let texture = font_texture(font, text, &self.ttf, &texture_creator)?;
        let query = texture.query();
        Ok((query.width as i32, query.height as i32))
    }

    fn fill_text<P: AsRef<std::path::Path>, Text: AsRef<str>>(
        &mut self,
        font: P,
        text: Text,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let texture = font_texture(font, text, &self.ttf, &texture_creator)?;
        self.canvas.copy(
            &texture,
            None,
            Some(Rect::new(x, y, width as u32, height as u32)),
        )?;
        Ok(())
    }

    fn clear(&mut self, rgb: &Rgb) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(rgb.0, rgb.1, rgb.2));
        self.canvas.clear();
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "key")]
enum Key {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Up,
    Down,
    Left,
    Right,
    Enter,
    Backspace,
    Space,
}

impl Key {
    fn from_sdl_keycode(keycode: Keycode) -> Option<Key> {
        let v = match keycode {
            Keycode::Num0 | Keycode::Kp0 => Key::Zero,
            Keycode::Num1 | Keycode::Kp1 => Key::One,
            Keycode::Num2 | Keycode::Kp2 => Key::Two,
            Keycode::Num3 | Keycode::Kp3 => Key::Three,
            Keycode::Num4 | Keycode::Kp4 => Key::Four,
            Keycode::Num5 | Keycode::Kp5 => Key::Five,
            Keycode::Num6 | Keycode::Kp6 => Key::Six,
            Keycode::Num7 | Keycode::Kp7 => Key::Seven,
            Keycode::Num8 | Keycode::Kp8 => Key::Eight,
            Keycode::Num9 | Keycode::Kp9 => Key::Nine,
            Keycode::A => Key::A,
            Keycode::B => Key::B,
            Keycode::C => Key::C,
            Keycode::D => Key::D,
            Keycode::E => Key::E,
            Keycode::F => Key::F,
            Keycode::G => Key::G,
            Keycode::H => Key::H,
            Keycode::I => Key::I,
            Keycode::J => Key::J,
            Keycode::K => Key::K,
            Keycode::L => Key::L,
            Keycode::M => Key::M,
            Keycode::N => Key::N,
            Keycode::O => Key::O,
            Keycode::P => Key::P,
            Keycode::Q => Key::Q,
            Keycode::R => Key::R,
            Keycode::S => Key::S,
            Keycode::T => Key::T,
            Keycode::U => Key::U,
            Keycode::V => Key::V,
            Keycode::W => Key::W,
            Keycode::X => Key::X,
            Keycode::Y => Key::Y,
            Keycode::Z => Key::Z,
            Keycode::Up => Key::Up,
            Keycode::Down => Key::Down,
            Keycode::Left => Key::Left,
            Keycode::Right => Key::Right,
            Keycode::Return => Key::Enter,
            Keycode::Backspace => Key::Backspace,
            Keycode::Space => Key::Space,
            _ => return None,
        };
        Some(v)
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Key::Zero => "0",
            Key::One => "1",
            Key::Two => "2",
            Key::Three => "3",
            Key::Four => "4",
            Key::Five => "5",
            Key::Six => "6",
            Key::Seven => "7",
            Key::Eight => "8",
            Key::Nine => "9",
            Key::A => "A",
            Key::B => "B",
            Key::C => "C",
            Key::D => "D",
            Key::E => "E",
            Key::F => "F",
            Key::G => "G",
            Key::H => "H",
            Key::I => "I",
            Key::J => "J",
            Key::K => "K",
            Key::L => "L",
            Key::M => "M",
            Key::N => "N",
            Key::O => "O",
            Key::P => "P",
            Key::Q => "Q",
            Key::R => "R",
            Key::S => "S",
            Key::T => "T",
            Key::U => "U",
            Key::V => "V",
            Key::W => "W",
            Key::X => "X",
            Key::Y => "Y",
            Key::Z => "Z",
            Key::Up => "Up",
            Key::Down => "Down",
            Key::Left => "Left",
            Key::Right => "Right",
            Key::Enter => "Enter",
            Key::Backspace => "Backspace",
            Key::Space => "Space",
        };
        write!(f, "{val}")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    reimtris1_feature_parity: bool,
    restart: Vec<Key>,
    left: Vec<Key>,
    right: Vec<Key>,
    rotate_cw: Vec<Key>,
    rotate_ccw: Vec<Key>,
    soft_drop: Vec<Key>,
    hard_drop: Vec<Key>,
    swap: Vec<Key>,
    pause: Vec<Key>,
    toggle_mute: Vec<Key>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            reimtris1_feature_parity: false,
            restart: vec![Key::Enter, Key::Space],
            left: vec![Key::Left],
            right: vec![Key::Right],
            rotate_cw: vec![Key::X],
            rotate_ccw: vec![Key::Z],
            soft_drop: vec![Key::Down],
            hard_drop: vec![Key::Space],
            swap: vec![Key::C],
            pause: vec![Key::P],
            toggle_mute: vec![Key::M],
        }
    }
}

fn config_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Config, String> {
    let Some(config) = fs::read_to_string(path.as_ref()).ok() else {
        let config = Config::default();
        {
            println!("could not get config! creating default...");
            let config = toml::to_string(&config).map_err(|err| err.to_string())?;
            fs::write(path.as_ref(), config).map_err(|err| err.to_string())?;
            println!("created config at '{}'", path.as_ref().display());
        }
        return Ok(config);
    };
    let Some(config) = toml::from_str(&config).ok() else {
        println!("womp womp, config contains an invalid config, resetting...");
        let config = Config::default();
        {
            let config = toml::to_string(&config).map_err(|err| err.to_string())?;
            fs::write(path.as_ref(), config).map_err(|err| err.to_string())?;
            println!("created config at '{}'", path.as_ref().display());
        }
        return Ok(config);
    };
    Ok(config)
}

pub fn start_game() -> Result<(), String> {
    let mut game = Game::new();
    let mut actions = ActionsHeld::new();
    let mut paused = false;

    let config = {
        let base = xdg::BaseDirectories::new().map_err(|err| err.to_string())?;
        let path = base
            .place_config_file("reimtris2/config.toml")
            .map_err(|err| err.to_string())?;
        let config = config_from_file(path)?;
        config
    };

    const FONT: &'static str = "resources/josenfin_sans_regular.ttf";

    let audio_thread = audio::audio_thread();

    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("reimtris2", 1000, 800)
        .resizable()
        .maximized()
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let mut ctx = SdlUiCtx {
        canvas,
        ttf: ttf_context,
    };
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        ctx.clear(&Rgb(16, 16, 16))?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running Ok(()),
                Event::MouseMotion { .. } => {
                    if config.reimtris1_feature_parity {
                        break 'running Ok(());
                    }
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    let Some(key) = Key::from_sdl_keycode(keycode) else {
                        continue;
                    };
                    if config.pause.contains(&key) {
                        paused = !paused;
                    };
                    if config.restart.contains(&key) && !paused && game.game_over {
                        game = Game::new();
                    }
                    if config.toggle_mute.contains(&key) {
                        audio_thread.send(audio::Command::ToggleMuted).unwrap();
                    }
                    if config.left.contains(&key) {
                        actions.insert(Action::Left, game.ticks);
                    }
                    if config.right.contains(&key) {
                        actions.insert(Action::Right, game.ticks);
                    }
                    if config.soft_drop.contains(&key) {
                        actions.insert(Action::SoftDrop, game.ticks);
                    }
                    if config.hard_drop.contains(&key) {
                        actions.insert(Action::HardDrop, game.ticks);
                    }
                    if config.rotate_cw.contains(&key) {
                        actions.insert(Action::RotateCw, game.ticks);
                    }
                    if config.rotate_ccw.contains(&key) {
                        actions.insert(Action::RotateCcw, game.ticks);
                    }
                    if config.swap.contains(&key) {
                        actions.insert(Action::Swap, game.ticks);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => {
                    let Some(key) = Key::from_sdl_keycode(keycode) else {
                        continue;
                    };
                    if config.left.contains(&key) {
                        actions.remove(&Action::Left);
                    }
                    if config.right.contains(&key) {
                        actions.remove(&Action::Right);
                    }
                    if config.soft_drop.contains(&key) {
                        actions.remove(&Action::SoftDrop);
                    }
                    if config.hard_drop.contains(&key) {
                        actions.remove(&Action::HardDrop);
                    }
                    if config.rotate_cw.contains(&key) {
                        actions.remove(&Action::RotateCw);
                    }
                    if config.rotate_ccw.contains(&key) {
                        actions.remove(&Action::RotateCcw);
                    }
                    if config.swap.contains(&key) {
                        actions.remove(&Action::Swap);
                    }
                }
                _ => {}
            }
        }

        ctx.draw_board(&game.board, &game.current_tetromino)?;
        ctx.draw_bag(&game.held_tetromino, &game.next_tetrominos)?;
        ctx.draw_score(FONT, &game.score)?;

        if paused {
            let keys = config
                .pause
                .iter()
                .map(|v| v.to_string().to_lowercase())
                .collect::<Vec<_>>()
                .join(" | ");
            let paused = format!("game paused o_o... press [{keys}] to unpause !!");

            ctx.draw_important_text(FONT, paused)?;
        } else if game.game_over {
            let keys = config
                .restart
                .iter()
                .map(|v| v.to_string().to_lowercase())
                .collect::<Vec<_>>()
                .join(" | ");

            let game_over = format!("game over T_T... press [{keys}] 2 restart :D");
            ctx.draw_important_text(FONT, game_over)?;
        } else {
            let effects = game.step(&actions);
            effects.into_iter().for_each(|effect| {
                audio_thread
                    .send(audio::Command::PlayEffect(effect))
                    .unwrap()
            });
        }

        ctx.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
