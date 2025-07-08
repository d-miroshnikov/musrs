use std::{fs::File, path::Path, time::Duration};

use lofty::{file::TaggedFileExt, read_from_path, tag::ItemKey};
use rodio::{Decoder, Source};

pub struct MusicInfo {
    pub file_name: String,
    pub path: String,
    pub meta: AudioMeta,
    pub source: Decoder<File>,
}

pub struct AudioMeta {
    artist: String,
    title: String,
    duration: Option<f32>,
}

pub fn load_music(path: &str) -> MusicInfo {
    let file = File::open(path).unwrap();
    let source = Decoder::new(file).unwrap();

    let track_duration: Duration = source.total_duration().unwrap();
    let mut audio_meta = get_audio_metadata(&path);
    audio_meta.duration = Some(track_duration.as_secs_f32());

    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap();

    let parts: Vec<&str> = file_name.split(".").collect();

    return MusicInfo {
        path: path.to_string(),
        meta: audio_meta,
        file_name: parts[0].to_owned(),
        source: source,
    };
}

fn get_audio_metadata(path: &str) -> AudioMeta {
    let tagged = read_from_path(path).unwrap();
    let binding = tagged.primary_tag();

    let artist = binding
        .and_then(|tag| tag.get_string(&ItemKey::TrackArtists))
        .unwrap_or("Unknown artist");

    let title = binding
        .and_then(|tag| tag.get_string(&ItemKey::TrackTitle))
        .unwrap_or("Unknown artist");

    return AudioMeta {
        artist: artist.to_string(),
        title: title.to_string(),
        duration: None,
    };
}
