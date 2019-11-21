mod v1_2;

pub mod prelude {
    pub use super::v1_2::prelude as save_data_v1_2;
    pub use super::LevelSaveData;
    pub use super::MusicData;
    pub use super::SavefileData;
    pub use super::StatsData;
}

use std::collections::HashMap;

use climer::Time;

use crate::audio::Music;
use crate::level_manager::Level;
use crate::resources::prelude::CheckpointData;

#[derive(Deserialize, Serialize, Default)]
pub struct SavefileData {
    pub levels: HashMap<Level, LevelSaveData>,
}

impl SavefileData {
    pub fn level(&self, target: &Level) -> Option<&LevelSaveData> {
        self.levels.get(target)
    }
}

#[derive(Deserialize, Serialize)]
pub struct LevelSaveData {
    pub checkpoint: Option<CheckpointData>,
    pub music:      MusicData,
    pub stats:      StatsData,
    pub best_time:  Option<Time>,
    pub won:        bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MusicData {
    pub queue: Vec<usize>,
}

impl From<&Music> for MusicData {
    fn from(music: &Music) -> Self {
        let mut queue = music.queue.clone();
        // If there is a last played song, save that song to queue as well.
        // Next time the savefile is loaded, the last_played song will be the first to play.
        if let Some(last_played) = music.last_played.as_ref() {
            queue.push(*last_played);
        }
        Self { queue }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatsData {
    pub player_deaths: u32,
}

impl From<v1_2::SavefileData> for SavefileData {
    fn from(old: v1_2::SavefileData) -> Self {
        use std::convert::TryFrom;

        Self {
            levels: old
                .levels
                .into_iter()
                .filter_map(|(old_level_filename, old_level_data)| {
                    match Level::try_from(old_level_filename.as_str()) {
                        Ok(level) => Some((level, LevelSaveData {
                            checkpoint: old_level_data.checkpoint,
                            music:      MusicData {
                                queue: old_level_data.music.queue,
                            },
                            stats:      StatsData {
                                player_deaths: old_level_data
                                    .stats
                                    .player_deaths,
                            },
                            best_time:  old_level_data.best_time,
                            won:        old_level_data.won,
                        })),
                        Err(e) => {
                            eprintln!(
                                "Failed converting v1.2 level data, skipping: \
                                 {:#?}",
                                e
                            );
                            None
                        }
                    }
                })
                .collect(),
        }
    }
}
