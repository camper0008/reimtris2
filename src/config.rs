use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "key")]
pub enum Key {
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
    pub fn from_sdl2_keycode(keycode: sdl2::keyboard::Keycode) -> Option<Key> {
        use sdl2::keyboard::Keycode;
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
pub struct Config {
    pub reimtris1_feature_parity: bool,
    pub restart: Vec<Key>,
    pub left: Vec<Key>,
    pub right: Vec<Key>,
    pub rotate_cw: Vec<Key>,
    pub rotate_ccw: Vec<Key>,
    pub soft_drop: Vec<Key>,
    pub hard_drop: Vec<Key>,
    pub swap: Vec<Key>,
    pub pause: Vec<Key>,
    pub toggle_mute: Vec<Key>,
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

impl Config {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Config, String> {
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
}
