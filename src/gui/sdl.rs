use crate::actions::{Action, ActionsHeld};
use crate::config::{Config, Key};
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

struct SdlUiCtx<'a> {
    canvas: &'a mut WindowCanvas,
    ttf: Sdl2TtfContext,
}

impl SdlUiCtx<'_> {
    fn present(&mut self) {
        self.canvas.present();
    }

    fn font_texture<'font, 'a, P: AsRef<std::path::Path>, Text: AsRef<str>, C>(
        &self,
        font: P,
        text: Text,
        texture_creator: &'font TextureCreator<C>,
    ) -> Result<Texture<'font>, String> {
        let font = self.ttf.load_font(font, 24)?;
        let game_over_text = font
            .render(text.as_ref())
            .solid(Color::RGB(255, 255, 255))
            .map_err(|err| err.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(game_over_text)
            .map_err(|err| err.to_string())?;

        Ok(texture)
    }
}

impl UiCtx<String> for SdlUiCtx<'_> {
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
        let texture = self.font_texture(font, text, &texture_creator)?;
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
        let texture = self.font_texture(font, text, &texture_creator)?;
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

pub fn start_game(config: Config) -> Result<(), String> {
    let mut game = Game::new();
    let mut actions = ActionsHeld::new();
    let mut paused = false;

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

    let mut canvas = window.into_canvas().build().unwrap();
    let mut ctx = SdlUiCtx {
        canvas: &mut canvas,
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
                    let Some(key) = Key::from_sdl2_keycode(keycode) else {
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
                    let Some(key) = Key::from_sdl2_keycode(keycode) else {
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
