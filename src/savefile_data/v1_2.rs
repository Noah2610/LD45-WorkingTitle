use std::collections::HashMap;

use climer::Time;

use crate::resources::prelude::CheckpointData;

pub mod prelude {
    pub use super::LevelManagerData;
    pub use super::LevelSaveData;
    pub use super::MusicData;
    pub use super::SavefileData;
    pub use super::StatsData;
}

#[derive(Deserialize, Serialize, Default)]
pub struct SavefileData {
    pub levels: HashMap<String, LevelSaveData>,
}

#[derive(Deserialize, Serialize)]
pub struct LevelSaveData {
    pub level_manager: LevelManagerData,
    pub checkpoint:    Option<CheckpointData>,
    pub music:         MusicData,
    pub stats:         StatsData,
    pub best_time:     Option<Time>,
    pub won:           bool,
}

#[derive(Deserialize, Serialize)]
pub struct LevelManagerData {
    pub level_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MusicData {
    pub queue: Vec<usize>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatsData {
    pub player_deaths: u32,
}
