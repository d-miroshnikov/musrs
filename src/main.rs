pub mod formater;
pub mod loaders;

use std::{
    io::{self},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use rodio::{OutputStream, Sink, Source};

use clap::Parser;
use crossterm::{
    ExecutableCommand,
    cursor::{MoveLeft, MoveTo, MoveUp, RestorePosition, SavePosition},
    terminal::{Clear, ClearType},
};

use crate::loaders::{
    load_meta_from_path::load_meta_from_path,
    load_music::{MusicInfo, load_music},
};

use crate::formater::time_formatter::format_to_time;

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
    // println!("path: {:?}", path);

    let metadata;

    match load_meta_from_path(&path) {
        Ok(meta) => metadata = meta,
        Err(e) => {
            println!("{}", e);
            return;
        }
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

    let music = load_music(&path);
    spawn_thread(music);

    let running = Arc::new(AtomicBool::new(true));

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    running.store(false, Ordering::Relaxed);
}

fn spawn_thread(music_info: MusicInfo) {
    let track_duration: Duration = music_info.source.total_duration().unwrap();
    let bar_len = 50;

    thread::spawn(|| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(music_info.source.amplify(0.3));
        sink.sleep_until_end();
    });

    print!("\n\n");

    thread::spawn(move || {
        for i in 0..=track_duration.as_secs() {
            let percent = i as f32 / track_duration.as_secs() as f32;
            let filled_len = (bar_len as f32 * percent) as usize;
            let bar = "=".repeat(filled_len) + &"-".repeat(bar_len - filled_len);

            io::stdout()
                .execute(SavePosition)
                .unwrap()
                .execute(MoveUp(3))
                .unwrap()
                .execute(MoveLeft(10000))
                .unwrap()
                .execute(Clear(ClearType::CurrentLine))
                .unwrap();

            println!(
                "Now playing {}\n\n[{}] {:>3}|{}",
                music_info.file_name,
                bar,
                format_to_time(i),
                format_to_time(track_duration.as_secs())
            );

            /* println!(
                "Now playing {} - {}\n\n[{}] {:>3}|{}",
                music_info.meta.artist,
                music_info.meta.title,
                bar,
                format_to_form(i),
                format_to_form(track_duration.as_secs())
            ); */

            io::stdout().execute(RestorePosition).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
