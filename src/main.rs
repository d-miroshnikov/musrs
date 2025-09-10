pub mod audio_handler;
pub mod formater;
pub mod loaders;

use std::{
    fs::File,
    io::{self},
    path::Path,
    time::Duration,
};

use clap::Parser;
use rodio::{Decoder, Source};

use crate::{
    audio_handler::{
        audio_handler::{MusicInfoTemp, MusicTimeBarHandler, SimpleAudioHandler},
        errors::{EmptyPlaylistError, TrackIsPaused, TrackIsPlaying},
    },
    loaders::load_meta_from_path::load_meta_from_path,
};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Cli::parse();

    let path = args.path;
    let metadata;
    match load_meta_from_path(&path) {
        Ok(meta) => metadata = meta,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let mut music_arr: Vec<MusicInfoTemp> = Vec::new();
    if metadata.is_file() {
        let music_info = get_music_info(&path);

        music_arr.push(music_info);
    }
    // is file or dir
    // file - play only one track
    // dir - play it like playlist
    // add play/stop
    // add some cmd:
    // 1. next
    // 2. prev
    // 3. repeat
    // 4. options

    let music_time_bar_handler = MusicTimeBarHandler::new();
    let mut audio_handler = SimpleAudioHandler::new(music_arr, music_time_bar_handler);
    let _ = audio_handler.start();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();
        match cmd {
            "add" => {}
            "play" => match audio_handler.play() {
                Ok(()) => (),
                Err(err) => {
                    if err.is::<TrackIsPlaying>() {
                        println!("track is already playing")
                    }
                    if err.is::<EmptyPlaylistError>() {
                        println!("playlist is empty")
                    }
                }
            },
            "pause" => match audio_handler.pause() {
                Ok(()) => (),
                Err(err) => {
                    if err.is::<TrackIsPaused>() {
                        println!("track is already paused")
                    }
                    if err.is::<EmptyPlaylistError>() {
                        println!("playlist is empty")
                    }
                }
            },
            "stop" => {
                break;
            }
            "clear" => match audio_handler.clear_history() {
                Ok(()) => (),
                Err(err) => {
                    if err.is::<EmptyPlaylistError>() {
                        println!("playlist is empty")
                    }
                }
            }
            _ => {
                println!("Undefined command: {}", cmd);
            }
        }
    }
}

fn get_music_info(path: &str) -> MusicInfoTemp {
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap();

    let parts: Vec<&str> = file_name.split(".").collect();

    let file = File::open(path).unwrap();
    let source = Decoder::new(file).unwrap();
    let duration: Duration = source.total_duration().unwrap();

    return MusicInfoTemp {
        path: path.to_string(),
        title: parts[0].to_string(),
        duration: duration.as_secs(),
    };
}
