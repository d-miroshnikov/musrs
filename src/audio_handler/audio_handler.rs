use std::{
    env::current_exe,
    error::Error,
    fs::File,
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand,
    cursor::{MoveLeft, MoveUp, RestorePosition, SavePosition},
    terminal::{Clear, ClearType},
};
use rodio::{Decoder, OutputStream, Sink};

use crate::{
    audio_handler::errors::{EmptyPlaylistError, TrackIsPaused, TrackIsPlaying},
    formater::time_formatter::format_to_time,
};

pub struct MusicInfoTemp {
    pub path: String,
    pub title: String,
    pub duration: u64,
}

pub struct SimpleAudioHandler {
    bar_control: MusicTimeBarHandler,
    current_pos: i32,
    playlist: Vec<MusicInfoTemp>,
    player: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    // queue: SourcesQueueOutput<f32>,
}

impl SimpleAudioHandler {
    pub fn new(playlist: Vec<MusicInfoTemp>, bar_control: MusicTimeBarHandler) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        // let (sink, queue) = Sink::new_idle();

        Self {
            bar_control,
            current_pos: 0,
            playlist,
            player: Arc::new(Mutex::new(sink)),
            _stream,
            // queue,
        }
    }

    pub fn start(&self) -> Result<(), Box<EmptyPlaylistError>> {
        let index = self.current_pos as usize;
        let current_music = match self.playlist.get(index) {
            Some(music) => music,
            None => return Err(Box::new(EmptyPlaylistError)),
        };

        self.add_track(current_music);
        self.bar_control.create_bar(current_music);

        Ok(())
    }

    pub fn add_track(&self, mit: &MusicInfoTemp) {
        let player = Arc::clone(&self.player);
        let locked_player = player.lock().unwrap();

        let file = File::open(&mit.path).unwrap();
        let source = Decoder::new(file).unwrap();

        locked_player.append(source);
        locked_player.set_volume(0.5);
    }

    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        let player = Arc::clone(&self.player);
        let locked_player = player.lock().unwrap();

        if locked_player.empty() {
            return Err(Box::new(EmptyPlaylistError));
        }

        if locked_player.is_paused() {
            locked_player.play();
        } else {
            return Err(Box::new(TrackIsPlaying));
        }

        Ok(())
    }

    pub fn pause(&self) -> Result<(), Box<dyn Error>> {
        let player = Arc::clone(&self.player);
        let locked_player = player.lock().unwrap();

        if locked_player.empty() {
            return Err(Box::new(EmptyPlaylistError));
        }

        if !locked_player.is_paused() {
            locked_player.pause();
        } else {
            return Err(Box::new(TrackIsPaused));
        }

        Ok(())
    }
}

pub struct MusicTimeBarHandler {
    state: String,
}

impl MusicTimeBarHandler {
    pub fn new() -> Self {
        Self {
            state: "play".to_string(),
        }
    }

    pub fn create_bar(&self, mit: &MusicInfoTemp) {
        let bar_len = 50;

        let state: String = self.state.clone();
        let duration = mit.duration.clone();
        let track_title = mit.title.clone();

        println!("\n");

        thread::spawn(move || {
            if state == "play" {
                for i in 0..=duration {
                    let percent = i as f32 / duration as f32;
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
                        track_title,
                        bar,
                        format_to_time(i),
                        format_to_time(duration)
                    );

                    io::stdout().execute(RestorePosition).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            }
        });
    }

    pub fn play(&mut self) {
        self.state = "play".to_string();
    }

    pub fn pause(&mut self) {
        self.state = "pause".to_string();
    }
}

/* trait AudioDataHandler {
    fn get_artist(path: &str) -> String;
    fn get_title(path: &str) -> String;
    fn get_audio_title(path: &str) -> String;
}

struct SimpleAudioDataHandler {}

impl AudioDataHandler for SimpleAudioHandler {} */
