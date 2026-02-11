use std::{
    error::Error,
    fs::File,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use rodio::{Decoder, OutputStream, Sink, Source};

use crate::audio_player::errors::{EmptyPlaylistError, TrackIsPaused, TrackIsPlaying};

// --------------------------------------- //
pub struct MusicInfoTemp {
    pub str_path: String,
}

impl MusicInfoTemp {
    pub fn new(path: String) -> Self {
        Self { str_path: path }
    }

    // ----------------------------- //
    // Internal API
    fn get_path(&self) -> &Path {
        let path = Path::new(&self.str_path);
        return &path;
    }

    fn get_file(&self) -> File {
        let path = self.str_path.clone();
        let file = File::open(path).unwrap();

        return file;
    }

    // ----------------------------- //
    // Public API
    pub fn get_title(&self) -> String {
        let path = self.get_path();

        let file_name = 
            path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap();

        let parts: Vec<&str> = file_name.split(".").collect();

        return parts[0].to_string();
    }

    pub fn get_source(&self) -> Decoder<File> {
        let file = self.get_file();
        let source = Decoder::new(file).unwrap();

        return source;
    }

    pub fn get_duration(&self) -> u64 {
        let source = self.get_source();
        let duration: Duration = source.total_duration().unwrap();

        return duration.as_secs();
    }
}

// ------------------------------------- //

pub trait Timebar {
    fn new() -> Self;
    fn create_bar(&self, mit: &MusicInfoTemp);
    fn play(&self);
    fn pause(&self);
    fn clear(&self);
}

pub struct SimpleAudioPlayer<T> {
    timebar: T,
    current_pos: i32,
    playlist: Vec<MusicInfoTemp>,
    player: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    // queue: SourcesQueueOutput<f32>,
}

impl<T: Timebar> SimpleAudioPlayer<T> {
    pub fn new(playlist: Vec<MusicInfoTemp>, timebar_impl: T) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            timebar: timebar_impl,
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
        self.timebar.create_bar(current_music);

        Ok(())
    }

    pub fn add_track(&self, mit: &MusicInfoTemp) {
        let player = Arc::clone(&self.player);
        let locked_player = player.lock().unwrap();

        let source = mit.get_source();

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
            self.timebar.play();
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
            self.timebar.pause();
        } else {
            return Err(Box::new(TrackIsPaused));
        }

        Ok(())
    }

    pub fn clear_history(&mut self) -> Result<(), Box<dyn Error>> {
        self.timebar.clear();

        Ok(())
    }
}

/* trait AudioDataHandler {
    fn get_artist(path: &str) -> String;
    fn get_title(path: &str) -> String;
    fn get_audio_title(path: &str) -> String;
}

struct SimpleAudioDataHandler {}

impl AudioDataHandler for SimpleAudioPlayer {} */
