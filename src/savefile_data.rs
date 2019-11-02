use crate::audio::Music;
use crate::resources::prelude::CheckpointData;

pub mod prelude {
    pub use super::LevelManagerData;
    pub use super::MusicData;
    pub use super::SavefileData;
    pub use super::StatsData;
}

#[derive(Deserialize, Serialize)]
pub struct SavefileData {
    pub level_manager: LevelManagerData,
    pub checkpoint:    Option<CheckpointData>,
    pub music:         MusicData,
    pub stats:         StatsData,
}

#[derive(Deserialize, Serialize)]
pub struct LevelManagerData {
    pub level_name: String,
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
