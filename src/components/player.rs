use super::component_prelude::*;

pub struct PlayerJumpData {
    pub jump_strength:      f32,
    pub gravity:            Vector,
    pub jump_gravity:       Vector,
    pub decr_jump_strength: f32,
    pub min_jump_velocity:  f32,
}

pub struct Player {
    pub settings:     PlayerSettings,
    pub acceleration: Vector,
    pub max_velocity: (Option<f32>, Option<f32>),
    pub jump_data:    Option<PlayerJumpData>,
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}

impl From<PlayerSettings> for Player {
    fn from(settings: PlayerSettings) -> Player {
        Player {
            acceleration: settings.acceleration.clone(),
            max_velocity: settings.max_velocity.clone(),
            jump_data:    None,
            settings:     settings,
        }
    }
}
