use deathframe::geo::Vector;

pub mod prelude {
    pub use super::CameraSettings;
    pub use super::PlayerSettings;
    pub use super::Settings;
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub camera: CameraSettings,
    pub player: PlayerSettings,
}

#[derive(Clone, Deserialize)]
pub struct CameraSettings {
    pub size: Vector,
}

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:          Vector,
    pub acceleration:  Vector,
    pub max_velocity:  (Option<f32>, Option<f32>),
    pub decr_velocity: Vector,
    pub jump_strength: f32,
    pub gravity:       Vector,
    pub jump_gravity:  Vector,
}
