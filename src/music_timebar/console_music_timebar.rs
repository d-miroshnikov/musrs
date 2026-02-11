use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    formater::time_formatter::format_to_time,
};

use crate::MusicInfoTemp;
use crate::audio_handler::audio_handler::Timebar;

pub struct ConsoleMusicTimebar {
    state: Arc<Mutex<String>>,
    current_time: Arc<Mutex<u64>>,
}

impl Timebar for ConsoleMusicTimebar {
    fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new("play".to_string())),
            current_time: Arc::new(Mutex::new(0)),
        }
    }

    fn create_bar(&self, mit: &MusicInfoTemp) {
        let state = Arc::clone(&self.state);
        let current_time = Arc::clone(&self.current_time);

        let duration = mit.duration;
        let track_title = mit.title.clone();
        let bar_len = 50;

        thread::spawn(move || {
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

    fn play(&self) {
        let mut state_guard = self.state.lock().unwrap();
        *state_guard = "play".to_string();
    }

    fn pause(&self) {
        let mut state_guard = self.state.lock().unwrap();
        *state_guard = "pause".to_string();
    }

    fn clear(&self) {
        println("clear")
    }
}