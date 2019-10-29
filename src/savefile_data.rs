use crate::audio::Music;
use crate::resources::prelude::CheckpointData;

pub mod prelude {
    pub use super::MusicData;
    pub use super::SavefileData;
}

#[derive(Deserialize, Serialize)]
pub struct SavefileData {
    pub checkpoint: CheckpointData,
    pub music:      MusicData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MusicData {
    pub queue:       Vec<usize>,
    pub last_played: Option<usize>,
}

impl From<&Music> for MusicData {
    fn from(music: &Music) -> Self {
        Self {
            queue:       music.queue.clone(),
            last_played: music.last_played.clone(),
        }
    }
}
