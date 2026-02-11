pub mod audio_player;
pub mod formater;
pub mod loaders;
pub mod music_timebar;

use std::{
    io::{self},
};

use clap::Parser;

use crate::{
    audio_player::{
        simple_audio_player::{MusicInfoTemp, SimpleAudioPlayer, Timebar},
        errors::{EmptyPlaylistError, TrackIsPaused, TrackIsPlaying},
    },
    loaders::load_meta_from_path::load_meta_from_path,
};

use crate::music_timebar::crossterm_music_timebar::CrosstermMusicTimebar;

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
        let music_info = MusicInfoTemp::new(path.clone());

        music_arr.push(music_info);
    }

    let crossterm_music_timebar = CrosstermMusicTimebar::new();
    let mut simple_audio_player = match SimpleAudioPlayer::new(music_arr, crossterm_music_timebar) {
        Ok(sap) => sap,
        Err(e)=>{
            println!("{}", e);
            return;
        } 
    };

    let _ = simple_audio_player.start();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input_parts: Vec<&str> = input.trim().split(" ").collect();
        let cmd = input_parts[0];

        match cmd {
            "add" => {
                if input_parts.len() < 2 || input_parts.len() > 2 {
                    println!("musrs: incorrect usage of «add». Example: musrs add ./path/to/track");
                    continue;
                }

                let metadata;
                let path_to_track = input_parts[1];
                match load_meta_from_path(&path_to_track) {
                    Ok(meta) => metadata = meta,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }

                if !metadata.is_file() {
                    println!("musrs: provided path is not a file");
                    continue;
                }

                let music_info = MusicInfoTemp::new(path.clone());
                simple_audio_player.add_track(&music_info);

                println!("musrs: Track {} successfully added", music_info.get_title());
            }
            "play" => match simple_audio_player.play() {
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
            "pause" => match simple_audio_player.pause() {
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
            "clear" => match simple_audio_player.clear_history() {
                Ok(()) => (),
                Err(err) => {
                    if err.is::<EmptyPlaylistError>() {
                        println!("playlist is empty")
                    }
                }
            },
            _ => {
                println!("Undefined command: {}", cmd);
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(120));
    }
}