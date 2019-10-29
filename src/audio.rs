use amethyst::assets::Loader;
use amethyst::audio::{AudioSink, OggFormat, SourceHandle};
use amethyst::ecs::{World, WorldExt};

pub mod prelude {
    pub use super::initialize_music;
    pub use super::Music;
    pub use super::MUSIC_VOLUME;
}

pub const MUSIC_VOLUME: f32 = 1.0;
const SONG_FILES: &[&str] = &[
    "audio/song01.ogg",
    "audio/song02.ogg",
    "audio/song03.ogg",
    "audio/song04.ogg",
    "audio/song05.ogg",
    "audio/song06.ogg",
    "audio/song07.ogg",
    "audio/song08.ogg",
    "audio/song09.ogg",
    "audio/song10.ogg",
    "audio/song11.ogg",
];

#[derive(Default, Debug)]
pub struct Music {
    songs:           Vec<SourceHandle>,
    pub queue:       Vec<usize>,
    pub last_played: Option<usize>,
}

impl Music {
    pub fn new(songs: Vec<SourceHandle>) -> Self {
        Self {
            songs,
            queue: Vec::new(),
            last_played: None,
        }
    }

    pub fn set(&mut self, index: usize) {
        self.print_warning_if_invalid_index(index);
        self.queue.insert(0, index);
    }

    pub fn force_set(&mut self, index: usize) {
        self.print_warning_if_invalid_index(index);
        let new_queue = self
            .queue
            .iter()
            .filter(|i| **i <= index)
            .map(Clone::clone)
            .collect::<Vec<usize>>();
        self.queue = new_queue;
        self.last_played =
            self.last_played.and_then(|last| Some(index.min(last)));
    }

    pub fn current(&mut self) -> Option<SourceHandle> {
        if let Some(in_queue) = self.queue.pop() {
            self.last_played = Some(in_queue);
            self.songs.get(in_queue).map(Clone::clone)
        } else {
            if let Some(last) = self.last_played {
                self.songs.get(last).map(Clone::clone)
            } else {
                None
            }
        }
    }

    pub fn should_audio_stop(&self) -> bool {
        self.last_played.is_none()
    }

    pub fn clear(&mut self) {
        self.last_played = None;
        self.queue.clear();
    }

    fn print_warning_if_invalid_index(&self, index: usize) {
        if index >= self.songs.len() {
            eprintln!("WARNING: Given song index {} does not exist", index);
        }
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
