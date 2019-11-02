use amethyst::assets::{Loader, ProgressCounter};
use amethyst::audio::output::Output;
use amethyst::audio::{AudioSink, OggFormat, SourceHandle};
use amethyst::ecs::{World, WorldExt};

use crate::settings::prelude::*;

pub mod prelude {
    pub use super::initialize_music;
    pub use super::set_decreased_volume;
    pub use super::set_normal_volume;
    pub use super::stop_audio;
    pub use super::Music;
}

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

#[derive(Default)]
pub struct Music {
    songs:           Vec<SourceHandle>,
    progress:        ProgressCounter,
    paused:          bool,
    pub queue:       Vec<usize>,
    pub last_played: Option<usize>,
}

impl Music {
    pub fn new(songs: Vec<SourceHandle>, progress: ProgressCounter) -> Self {
        Self {
            songs,
            progress,
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.queue = Vec::new();
        self.last_played = None;
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
        if self.progress.is_complete() {
            if self.paused {
                self.get_last_played()
            } else {
                if let Some(in_queue) = self.queue.pop() {
                    self.last_played = Some(in_queue);
                    self.songs.get(in_queue).map(Clone::clone)
                } else {
                    self.get_last_played()
                }
            }
        } else {
            None
        }
    }

    pub fn should_audio_stop(&self) -> bool {
        self.last_played.is_none() && self.queue.is_empty()
    }

    pub fn clear(&mut self) {
        self.last_played = None;
        self.queue.clear();
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    fn get_last_played(&self) -> Option<SourceHandle> {
        if let Some(last) = self.last_played {
            self.songs.get(last).map(Clone::clone)
        } else {
            None
        }
    }

    fn print_warning_if_invalid_index(&self, index: usize) {
        if index >= self.songs.len() {
            eprintln!("WARNING: Given song index {} does not exist", index);
        }
    }
}

pub fn stop_audio(world: &mut World) {
    world.write_resource::<Music>().clear();

    let output = world.read_resource::<Output>();
    let mut sink = world.write_resource::<AudioSink>();
    sink.stop();
    *sink = AudioSink::new(&output);
}

pub fn set_normal_volume(world: &mut World) {
    let volume = world.read_resource::<Settings>().music.volume;
    set_volume(world, volume);
}

pub fn set_decreased_volume(world: &mut World) {
    let volume = world.read_resource::<Settings>().music.decreased_volume;
    set_volume(world, volume);
}

fn set_volume(world: &mut World, volume: f32) {
    world.write_resource::<AudioSink>().set_volume(volume);
}

pub fn initialize_music(world: &mut World) {
    let music_settings = world.read_resource::<Settings>().music.clone();

    let music = {
        let loader = world.read_resource::<Loader>();
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(music_settings.volume);

        let mut progress = ProgressCounter::new();

        let songs = SONG_FILES
            .iter()
            .map(|file| load_audio_track(&loader, &world, file, &mut progress))
            .collect::<Vec<_>>();
        Music::new(songs, progress)
    };

    world.insert(music);
}

// from `amethyst/examples/pong/audio.rs#18`
// Loads an ogg audio track.
fn load_audio_track(
    loader: &Loader,
    world: &World,
    file: &str,
    progress: &mut ProgressCounter,
) -> SourceHandle {
    use crate::helpers::resource;
    loader.load(resource(file), OggFormat, progress, &world.read_resource())
}
