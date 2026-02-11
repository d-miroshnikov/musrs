use std::{error::Error, fmt};

#[derive(Debug)]
pub struct EmptyPlaylistError;
impl fmt::Display for EmptyPlaylistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "playlist is empty")
    }
}
impl Error for EmptyPlaylistError {}

#[derive(Debug)]
pub struct TrackIsPlaying;
impl fmt::Display for TrackIsPlaying {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "track is playing")
    }
}
impl Error for TrackIsPlaying {}

#[derive(Debug)]
pub struct TrackIsPaused;
impl fmt::Display for TrackIsPaused {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "track is playing")
    }
}
impl Error for TrackIsPaused {}

#[derive(Debug)]
pub struct CannotCreateSimpleAudioPlayer;
impl fmt::Display for CannotCreateSimpleAudioPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot create simple audio player")
    }
}