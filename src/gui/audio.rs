use std::{fs::File, io::BufReader, sync::mpsc};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

use crate::game::SoundEffect;

fn source_from_path<P: AsRef<std::path::Path>>(path: P) -> Decoder<BufReader<File>> {
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    source
}

fn play_audio<P: AsRef<std::path::Path>>(
    stream_handle: &OutputStreamHandle,
    sink: &mut Option<Sink>,
    path: P,
    volume: f32,
) {
    let source = source_from_path(path);
    *sink = Sink::try_new(&stream_handle).ok();
    if let Some(sink) = sink {
        sink.set_volume(volume);
        sink.append(source);
    }
}

pub enum Command {
    ToggleMuted,
    PlayEffect(SoundEffect),
}

pub fn audio_thread() -> mpsc::Sender<Command> {
    let (sender, receiver) = mpsc::channel::<Command>();

    let _ = std::thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let music_sink = Sink::try_new(&stream_handle).unwrap();
        let mut hard_drop_sink = None;
        let mut line_clear_sink = None;
        let mut move_sink = None;
        let mut rotation_sink = None;
        let mut muted = false;

        music_sink.append(source_from_path("resources/music.ogg").repeat_infinite());

        loop {
            let Ok(cmd) = receiver.recv() else {
                break;
            };

            let effect = match cmd {
                Command::ToggleMuted => {
                    muted = !muted;
                    if muted {
                        music_sink.pause();
                    } else {
                        music_sink.play();
                    }
                    continue;
                }
                Command::PlayEffect(effect) => effect,
            };

            if muted {
                continue;
            }

            let base_volume = 0.5;
            match effect {
                SoundEffect::HardDrop => play_audio(
                    &stream_handle,
                    &mut hard_drop_sink,
                    "resources/hard_drop.ogg",
                    base_volume,
                ),
                SoundEffect::LineClear(lines_cleared) => play_audio(
                    &stream_handle,
                    &mut line_clear_sink,
                    "resources/line_clear.ogg",
                    base_volume + (lines_cleared as f32 - 1.0) * 0.5,
                ),
                SoundEffect::Move => play_audio(
                    &stream_handle,
                    &mut move_sink,
                    "resources/move.ogg",
                    base_volume,
                ),
                SoundEffect::Rotation => play_audio(
                    &stream_handle,
                    &mut rotation_sink,
                    "resources/rotation.ogg",
                    base_volume,
                ),
            };
        }
    });

    sender
}
