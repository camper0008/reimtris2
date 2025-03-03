use crate::actions::{Action, ActionsHeld};
use crate::game::Game;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
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

pub fn start_game() -> Result<(), String> {
    let mut game = Game::new();
    let mut actions = ActionsHeld::new();
    let mut paused = false;

    let audio_thread = audio::audio_thread();

    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("reimtris2", 1000, 800)
        .resizable()
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
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let keycode = match keycode {
                        Keycode::Return if !paused && game.game_over => {
                            game = Game::new();
                            continue;
                        }
                        Keycode::M => {
                            audio_thread.send(audio::Command::ToggleMuted).unwrap();
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

        ctx.draw_board(&game.board, &game.current_tetromino)?;

        if paused {
            ctx.draw_important_text(
                "resources/josenfin_sans_regular.ttf",
                "game paused o_o... press [p] to unpause !!",
            )?;
        } else if game.game_over {
            ctx.draw_important_text(
                "resources/josenfin_sans_regular.ttf",
                "game over T_T... press [enter] 2 restart :D",
            )?;
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
