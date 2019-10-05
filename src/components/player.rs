use super::component_prelude::*;

pub struct Player {
    pub settings:     PlayerSettings,
    pub acceleration: Vector,
    pub max_velocity: (Option<f32>, Option<f32>),
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}

impl From<PlayerSettings> for Player {
    fn from(settings: PlayerSettings) -> Player {
        Player {
            acceleration: settings.acceleration.clone(),
            max_velocity: settings.max_velocity.clone(),
            settings:     settings,
        }
    }
}
