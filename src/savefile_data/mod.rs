mod v1_2;

pub mod prelude {
    pub use super::initialize_savefile_data;
    pub use super::load_savefile_data;
    pub use super::save_savefile_data;
    pub use super::v1_2::prelude as save_data_v1_2;
    pub use super::LevelSaveData;
    pub use super::MusicData;
    pub use super::SavefileData;
    pub use super::StatsData;
}

use std::collections::HashMap;

use amethyst::ecs::{World, WorldExt};
use climer::Time;

use crate::audio::Music;
use crate::level_manager::Level;
use crate::resources::prelude::{CheckpointData, SavefileDataRes};
use crate::settings::prelude::{SavefileSettings, Settings};

pub fn initialize_savefile_data(world: &mut World) {
    let savefile_data_res = SavefileDataRes(load_savefile_data(
        &world.read_resource::<Settings>().savefile,
    ));

    world.insert(savefile_data_res);
}

pub fn load_savefile_data(
    savefile_settings: &SavefileSettings,
) -> Option<SavefileData> {
    use std::fs::File;
    use std::io::Read;

    let savefile_path = savefile_settings.path();
    if savefile_path.is_file() {
        let mut savefile_file = File::open(&savefile_path)
            .expect("Savefile should exist at this point");
        let mut savefile_raw = String::new();
        savefile_file.read_to_string(&mut savefile_raw).unwrap();
        match serde_json::de::from_str(&savefile_raw) {
            Ok(data) => Some(data),
            Err(e) => {
                eprintln!("Couldn't deserialize savefile data: {:#?}", e);
                eprintln!("Trying savefile data v1.2 ...");

                match serde_json::de::from_str::<v1_2::SavefileData>(
                    &savefile_raw,
                ) {
                    Ok(data_v1_2) => {
                        eprintln!("Found v1.2 savefile, converted to new data");
                        Some(data_v1_2.into())
                    }
                    Err(e) => {
                        eprintln!(
                            "Couldn't deserialize savefile data v1.2: {:#?}",
                            e
                        );
                        eprintln!("Not using a savefile");
                        None
                    }
                }
            }
        }
    } else {
        None
    }
}

pub fn save_savefile_data(world: &World) {
    use crate::helpers::write_file;

    if let Some(savefile_data) =
        world.read_resource::<SavefileDataRes>().0.as_ref()
    {
        match serde_json::to_string(savefile_data) {
            Ok(serialized) => {
                let savefile_settings =
                    &world.read_resource::<Settings>().savefile;
                let savefile_path = savefile_settings.path();
                write_file(savefile_path, serialized).unwrap();
            }
            Err(err) => eprintln!(
                "Couldn't save savefile data to file, an error occured while \
                 serializing save data:\n{:#?}",
                err
            ),
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct SavefileData {
    pub levels: HashMap<Level, LevelSaveData>,
}

impl SavefileData {
    pub fn level(&self, target: &Level) -> Option<&LevelSaveData> {
        self.levels.get(target)
    }

    pub fn has_completed_all_levels_except_very_easy(&self) -> bool {
        Level::iter().all(|level| {
            level == &Level::VeryEasy
                || self
                    .level(level)
                    .map(|level_save| level_save.won)
                    .unwrap_or(false)
        })
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
