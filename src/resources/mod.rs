pub mod prelude {
    pub use super::checkpoint::{CheckpointData, CheckpointRes};
    pub use super::reset_level::ResetLevel;
    pub use super::should_save::ShouldSave;
    pub use super::stop_audio::StopAudio;
    pub use super::win_game::WinGame;
    pub use crate::audio::Music;
}

mod checkpoint;
mod reset_level;
mod should_save;
mod stop_audio;
mod win_game;
