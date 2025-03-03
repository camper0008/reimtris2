use crate::actions::{Action, ActionsHeld};
use crate::board::Board;
use crate::game::Game;
use crate::Rgb;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

fn draw_board(canvas: &mut WindowCanvas, width: usize, height: usize, color: Rgb) {
    canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
    canvas
        .fill_rect(Rect::new(
            (800 - 24 * Board::WIDTH as i32) / 2 + width as i32 * 24,
            (600 - 24 * Board::HEIGHT as i32) / 2 + height as i32 * 24,
            24,
            24,
        ))
        .unwrap();
}

pub fn start_game() {
    let mut game = Game::new();
    let mut actions = ActionsHeld::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("reimtris2", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(16, 16, 16));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let keycode = match keycode {
                        Keycode::Left | Keycode::A => Action::Left,
                        Keycode::Right | Keycode::D => Action::Right,
                        Keycode::Down | Keycode::S => Action::SoftDrop,
                        Keycode::Space => Action::HardDrop,
                        Keycode::Z => Action::RotateCcw,
                        Keycode::X => Action::RotateCw,
                        Keycode::C => Action::Swap,
                        _ => continue,
                    };
                    actions.insert(keycode, game.ticks);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    let keycode = match keycode {
                        Keycode::Left | Keycode::A => Action::Left,
                        Keycode::Right | Keycode::D => Action::Right,
                        Keycode::Down | Keycode::S => Action::SoftDrop,
                        Keycode::Space => Action::HardDrop,
                        Keycode::Z => Action::RotateCcw,
                        Keycode::X => Action::RotateCw,
                        Keycode::C => Action::Swap,
                        _ => continue,
                    };
                    actions.remove(&keycode);
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::WHITE);
        canvas
            .draw_rect(Rect::new(
                (800 - 24 * Board::WIDTH as i32) / 2 - 1,
                (600 - 24 * Board::HEIGHT as i32) / 2 - 1,
                24 * Board::WIDTH as u32 + 2,
                24 * Board::HEIGHT as u32 + 2,
            ))
            .unwrap();

        for (y, row) in game.board.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                let color = match piece {
                    Some(t) => Rgb::from_tetromino(t),
                    None => Rgb(0, 0, 0),
                };
                draw_board(&mut canvas, x, y, color)
            }
        }

        let pattern = game
            .current_tetromino
            .tetromino
            .direction_pattern(&game.current_tetromino.direction);

        for (y, row) in pattern.iter().enumerate() {
            for x in row
                .iter()
                .enumerate()
                .filter(|(_, exists)| **exists)
                .map(|(x, _)| x)
            {
                let x = x as i8 + game.current_tetromino.x;
                let y = y as i8 + game.current_tetromino.y;

                if y < 0 {
                    continue;
                }

                if y >= Board::HEIGHT as i8 {
                    continue;
                }

                if x < 0 || x >= Board::WIDTH as i8 {
                    continue;
                }

                draw_board(
                    &mut canvas,
                    x as usize,
                    y as usize,
                    Rgb::from_tetromino(&game.current_tetromino.tetromino),
                )
            }
        }

        game.step(&actions);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
