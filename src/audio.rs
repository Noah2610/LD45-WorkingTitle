use std::collections::HashMap;

use amethyst::assets::Loader;
use amethyst::audio::{AudioSink, OggFormat, SourceHandle};
use amethyst::ecs::{World, WorldExt};

pub mod prelude {
    pub use super::initialize_music;
    pub use super::Music;
    pub use super::Song;
}

const SONG_FILES: &[(Song, &str)] = &[
    (Song::Song1, "audio/song1.ogg"),
    (Song::Song2, "audio/song2.ogg"),
];

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Song {
    Song1,
    Song2,
}

pub struct Music {
    songs:        HashMap<Song, SourceHandle>,
    current_song: Option<Song>,
}

impl Music {
    pub fn new(songs: HashMap<Song, SourceHandle>) -> Self {
        Self {
            songs,
            current_song: None,
        }
    }

    pub fn set(&mut self, song: Song) {
        self.current_song = Some(song);
    }

    pub fn current(&self) -> Option<SourceHandle> {
        if let Some(current) = self.current_song.as_ref() {
            self.songs.get(current).map(Clone::clone)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.current_song = None;
    }
}

pub fn initialize_music(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.5);

        let songs = SONG_FILES
            .iter()
            .map(|(song, file)| {
                (song.clone(), load_audio_track(&loader, &world, file))
            })
            .collect::<HashMap<_, _>>();
        Music::new(songs)
    };

    world.insert(music);
}

// from `amethyst/examples/pong/audio.rs#18`
// Loads an ogg audio track.
fn load_audio_track(
    loader: &Loader,
    world: &World,
    file: &str,
) -> SourceHandle {
    use crate::helpers::resource;
    loader.load(resource(file), OggFormat, (), &world.read_resource())
}
