pub mod prelude {
    pub use super::camera::CameraSettings;
    pub use super::debug::DebugSettings;
    pub use super::enemies::{EnemiesSettings, EnemySettings};
    pub use super::level_manager::{LevelManagerSettings, LevelSettings};
    pub use super::misc::MiscSettings;
    pub use super::music::MusicSettings;
    pub use super::player::{
        PlayerAnimationSizes,
        PlayerJumpSettings,
        PlayerSettings,
    };
    pub use super::savefile::SavefileSettings;
    pub use super::timer::TimerSettings;
    pub use super::Settings;
}

mod camera;
mod debug;
mod enemies;
mod level_manager;
mod misc;
mod music;
mod player;
mod savefile;
mod timer;

use prelude::*;

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub camera:        CameraSettings,
    pub player:        PlayerSettings,
    pub enemies:       EnemiesSettings,
    pub savefile:      SavefileSettings,
    pub level_manager: LevelManagerSettings,
    pub music:         MusicSettings,
    pub timer:         TimerSettings,
    pub misc:          MiscSettings,
    pub debug:         DebugSettings,
}
