pub mod prelude {
    pub use super::checkpoint::{CheckpointData, CheckpointRes};
    pub use super::reset_level::ResetLevel;
    pub use super::stop_audio::StopAudio;
    pub use super::win_game::WinGame;
    pub use crate::audio::Music;
}

mod checkpoint;
mod reset_level;
mod stop_audio;
mod win_game;
