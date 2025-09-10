use std::{
    error::Error,
    fs::File,
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossterm::{
    ExecutableCommand,
    cursor::{MoveTo, RestorePosition, SavePosition, position},
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

    pub fn play(&mut self) -> Result<(), Box<dyn Error>> {
        let player = Arc::clone(&self.player);
        let locked_player = player.lock().unwrap();

        if locked_player.empty() {
            return Err(Box::new(EmptyPlaylistError));
        }

        if locked_player.is_paused() {
            locked_player.play();
            self.bar_control.play();
        } else {
            return Err(Box::new(TrackIsPlaying));
        }

        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), Box<dyn Error>> {
        let player = Arc::clone(&self.player);
        let locked_player = player.lock().unwrap();

        if locked_player.empty() {
            return Err(Box::new(EmptyPlaylistError));
        }

        if !locked_player.is_paused() {
            locked_player.pause();
            self.bar_control.pause();
        } else {
            return Err(Box::new(TrackIsPaused));
        }

        Ok(())
    }

    pub fn clear_history(&mut self) -> Result<(), Box<dyn Error>> {
        self.bar_control.clear();

        Ok(())
    }
}

pub struct MusicTimeBarHandler {
    state: Arc<Mutex<String>>,
    current_time: Arc<Mutex<u64>>,
}

impl MusicTimeBarHandler {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new("play".to_string())),
            current_time: Arc::new(Mutex::new(0)),
        }
    }

    pub fn create_bar(&self, mit: &MusicInfoTemp) {
        let state = Arc::clone(&self.state);
        let current_time = Arc::clone(&self.current_time);

        let duration = mit.duration;
        let track_title = mit.title.clone();
        let bar_len = 50;

        thread::spawn(move || {
            io::stdout()
                .execute(Clear(ClearType::FromCursorUp)).unwrap()
                .execute(Clear(ClearType::FromCursorDown)).unwrap()
                .execute(MoveTo(0, 11)).unwrap();

            let formatted_duration = format_to_time(duration);

            loop {
                let is_playing = {
                    let state_guard = state.lock().unwrap();
                    *state_guard == "play"
                };

                if is_playing {
                    {
                        let mut time_guard = current_time.lock().unwrap();

                        if *time_guard > duration {
                            break;
                        }

                        let percent = *time_guard as f32 / duration as f32;
                        let filled_len = (bar_len as f32 * percent) as usize;
                        let bar = "=".repeat(filled_len) + &"-".repeat(bar_len - filled_len);

                        io::stdout()
                            .execute(SavePosition).unwrap()
                            .execute(MoveTo(0, 10)).unwrap()
                            .execute(Clear(ClearType::CurrentLine)).unwrap()
                            .execute(Clear(ClearType::FromCursorUp)).unwrap()
                            .execute(MoveTo(0, 3)).unwrap();
                        
                        println!(
                            "Now playing {}\n\n[{}] {:>3}|{}",
                            track_title,
                            bar,
                            format_to_time(*time_guard),
                            formatted_duration
                        );

                        io::stdout().execute(RestorePosition).unwrap();

                        *time_guard += 1;
                    }

                    thread::sleep(Duration::from_secs(1));
                } else {
                    thread::sleep(Duration::from_millis(50));
                }
            }
        });
    }

    pub fn play(&self) {
        let mut state_guard = self.state.lock().unwrap();
        *state_guard = "play".to_string();
    }

    pub fn pause(&self) {
        let mut state_guard = self.state.lock().unwrap();
        *state_guard = "pause".to_string();
    }

    pub fn clear(&self) {
        let current_pos = position().unwrap();

        for y in 10..current_pos.1 {
            io::stdout()
                .execute(MoveTo(0, y)).unwrap()
                .execute(Clear(ClearType::CurrentLine)).unwrap();
        }

        io::stdout()
            .execute(MoveTo(0, 11)).unwrap();
    }
}

/* trait AudioDataHandler {
    fn get_artist(path: &str) -> String;
    fn get_title(path: &str) -> String;
    fn get_audio_title(path: &str) -> String;
}

struct SimpleAudioDataHandler {}

impl AudioDataHandler for SimpleAudioHandler {} */
