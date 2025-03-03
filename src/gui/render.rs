use crate::actions::{Action, ActionsHeld};
use crate::board::Board;
use crate::game::{CurrentTetromino, Game};
use crate::Rgb;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
use std::time::Duration;

use super::audio::{self};

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 800;

fn draw_board_tile(
    canvas: &mut WindowCanvas,
    width: usize,
    height: usize,
    color: &Rgb,
    filled: bool,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
    let rect = Rect::new(
        center(24 * Board::WIDTH as i32, WIDTH) + width as i32 * 24,
        center(24 * Board::HEIGHT as i32, HEIGHT) + height as i32 * 24,
        24,
        24,
    );
    if filled {
        canvas.fill_rect(rect)
    } else {
        canvas.draw_rect(rect)
    }
}

fn center(length: i32, max: i32) -> i32 {
    (max - length) / 2
}

fn draw_tetromino_from_parts(
    canvas: &mut WindowCanvas,
    x: i8,
    y: i8,
    color: Rgb,
    pattern: [[bool; 4]; 4],
    filled: bool,
) -> Result<(), String> {
    for (y_offset, row) in pattern.iter().enumerate() {
        for x_offset in row
            .iter()
            .enumerate()
            .filter(|(_, exists)| **exists)
            .map(|(x, _)| x)
        {
            let x = x_offset as i8 + x;
            let y = y_offset as i8 + y;

            if y < 0 {
                continue;
            }

            draw_board_tile(canvas, x as usize, y as usize, &color, filled)?
        }
    }
    Ok(())
}

fn draw_board(
    canvas: &mut WindowCanvas,
    board: &Board,
    current: &CurrentTetromino,
) -> Result<(), String> {
    canvas.set_draw_color(Color::WHITE);
    canvas.draw_rect(Rect::new(
        center(24 * Board::WIDTH as i32, WIDTH) - 1,
        center(24 * Board::HEIGHT as i32, HEIGHT) - 1,
        24 * Board::WIDTH as u32 + 2,
        24 * Board::HEIGHT as u32 + 2,
    ))?;

    for (y, row) in board.iter().enumerate() {
        for (x, piece) in row.iter().enumerate() {
            let color = match piece {
                Some(t) => Rgb::from_tetromino(t),
                None => Rgb(0, 0, 0),
            };
            draw_board_tile(canvas, x, y, &color, true)?
        }
    }

    let pattern = current.tetromino.direction_pattern(&current.direction);

    draw_tetromino_from_parts(
        canvas,
        current.x,
        board.lowest_y(&current),
        Rgb(255, 255, 255),
        pattern,
        false,
    )?;

    draw_tetromino_from_parts(
        canvas,
        current.x,
        current.y,
        Rgb::from_tetromino(&current.tetromino),
        pattern,
        true,
    )?;

    Ok(())
}

fn font_texture<'font, 'a, C>(
    text: &'a str,
    ttf_context: &'a Sdl2TtfContext,
    texture_creator: &'font TextureCreator<C>,
) -> Result<Texture<'font>, String> {
    let font = ttf_context.load_font("resources/josenfin_sans_regular.ttf", 24)?;
    let game_over_text = font.render(text).solid(Color::RGB(255, 255, 255)).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(game_over_text)
        .unwrap();

    Ok(texture)
}

fn draw_important_text(
    text: &str,
    canvas: &mut WindowCanvas,
    ttf_context: &Sdl2TtfContext,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let texture = font_texture(text, &ttf_context, &texture_creator)?;

    let size = texture.query();
    let width = size.width;
    let height = size.height;

    let x = center(width as i32, WIDTH);
    let y = center(height as i32, HEIGHT);

    canvas.set_draw_color(Color::WHITE);
    canvas.draw_rect(Rect::new(x - 9, y - 9, width + 18, height + 18))?;

    canvas.set_draw_color(Color::RGB(16, 16, 16));
    canvas.fill_rect(Rect::new(x - 8, y - 8, width + 16, height + 16))?;
    canvas.copy(&texture, None, Some(Rect::new(x, y, width, height)))?;

    Ok(())
}

pub fn start_game() -> Result<(), String> {
    let mut game = Game::new();
    let mut actions = ActionsHeld::new();
    let mut paused = false;

    let audio_thread = audio::audio_thread();

    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("reimtris2", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::RGB(16, 16, 16));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running Ok(()),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let keycode = match keycode {
                        Keycode::Return if !paused && game.game_over => {
                            game = Game::new();
                            continue;
                        }
                        Keycode::P => {
                            paused = !paused;
                            continue;
                        }
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

        draw_board(&mut canvas, &game.board, &game.current_tetromino)?;

        if paused {
            draw_important_text(
                "game paused o_o... press [p] to unpause !!",
                &mut canvas,
                &ttf_context,
            )?;
        } else if game.game_over {
            draw_important_text(
                "game over T_T... press [enter] 2 restart :D",
                &mut canvas,
                &ttf_context,
            )?;
        } else {
            let effects = game.step(&actions);
            effects
                .into_iter()
                .for_each(|effect| audio_thread.send(effect).unwrap());
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
