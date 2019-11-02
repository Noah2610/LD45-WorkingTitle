use deathframe::geo::Vector;

pub mod prelude {
    pub use super::CameraSettings;
    pub use super::EnemiesSettings;
    pub use super::EnemySettings;
    pub use super::LevelManagerSettings;
    pub use super::MusicSettings;
    pub use super::PlayerAnimationSizes;
    pub use super::PlayerJumpSettings;
    pub use super::PlayerSettings;
    pub use super::SavefileSettings;
    pub use super::Settings;
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub camera:        CameraSettings,
    pub player:        PlayerSettings,
    pub enemies:       EnemiesSettings,
    pub savefile:      SavefileSettings,
    pub level_manager: LevelManagerSettings,
    pub music:         MusicSettings,
}

#[derive(Clone, Deserialize)]
pub struct CameraSettings {
    pub size: Vector,
}

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub normal_speed:    PlayerSpeedSettings,
    pub run_speed:       PlayerSpeedSettings,
    pub decr_velocity:   Vector,
    pub jump_data1:      PlayerJumpSettings,
    pub jump_data2:      PlayerJumpSettings,
    pub animation_sizes: PlayerAnimationSizes,
    pub slide_velocity:  f32,
    pub hover_velocity:  f32,
}

#[derive(Clone, Deserialize)]
pub struct PlayerSpeedSettings {
    pub acceleration: Vector,
    pub max_velocity: (Option<f32>, Option<f32>),
}

#[derive(Clone, Deserialize)]
pub struct PlayerJumpSettings {
    pub jump_strength:      f32,
    pub wall_jump_strength: Vector,
    pub dash_strength:      Vector,
    pub gravity:            Vector,
    pub jump_gravity:       Vector,
    pub decr_jump_strength: f32,
    pub min_jump_velocity:  f32,
    pub decr_velocity:      Vector,
    pub bounce_strength:    f32,
}

#[derive(Clone, Deserialize)]
pub struct PlayerAnimationSizes {
    pub no_sprite:       Vector,
    pub single_sprite:   Vector,
    pub animated_sprite: Vector,
}

#[derive(Clone, Deserialize)]
pub struct EnemiesSettings {
    pub ground: EnemySettings,
    pub flying: EnemySettings,
}

#[derive(Clone, Deserialize)]
pub struct EnemySettings {
    pub size:         Vector,
    pub gravity:      Option<Vector>,
    pub acceleration: Vector,
    pub max_velocity: (Option<f32>, Option<f32>),
}

#[derive(Clone, Deserialize)]
pub struct SavefileSettings {
    pub filename: String,
}

#[derive(Clone, Deserialize)]
pub struct LevelManagerSettings {
    pub level_names: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct MusicSettings {
    pub volume:           f32,
    pub decreased_volume: f32,
}
