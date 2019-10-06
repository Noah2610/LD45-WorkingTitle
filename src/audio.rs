use amethyst::assets::Loader;
use amethyst::audio::{AudioSink, OggFormat, SourceHandle};
use amethyst::ecs::{World, WorldExt};

pub mod prelude {
    pub use super::initialize_music;
    pub use super::Music;
    pub use super::MUSIC_VOLUME;
}

pub const MUSIC_VOLUME: f32 = 1.0;
const SONG_FILES: &[&str] = &["audio/song1.ogg", "audio/song2.ogg"];

#[derive(Default)]
pub struct Music {
    songs:        Vec<SourceHandle>,
    current_song: Option<usize>,
}

impl Music {
    pub fn new(songs: Vec<SourceHandle>) -> Self {
        Self {
            songs,
            current_song: None,
        }
    }

    pub fn set(&mut self, index: usize) {
        if index >= self.songs.len() {
            eprintln!("WARNING: Given song index {} does not exist", index);
        }
        self.current_song = Some(index);
    }

    pub fn current(&self) -> Option<SourceHandle> {
        if let Some(current) = self.current_song {
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
        sink.set_volume(MUSIC_VOLUME);

        let songs = SONG_FILES
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>();
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
