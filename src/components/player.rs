use super::component_prelude::*;

pub struct Player {
    pub settings:     PlayerSettings,
    pub acceleration: Vector,
    pub max_velocity: (Option<f32>, Option<f32>),
    pub jump_data:    Option<PlayerJumpSettings>,
    pub used_dash:    bool,
}

impl Player {
    pub fn set_normal_speed(&mut self) {
        self.acceleration = self.settings.normal_speed.acceleration.clone();
        self.max_velocity = self.settings.normal_speed.max_velocity.clone();
    }

    pub fn set_run_speed(&mut self) {
        self.acceleration = self.settings.run_speed.acceleration.clone();
        self.max_velocity = self.settings.run_speed.max_velocity.clone();
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}

impl From<PlayerSettings> for Player {
    fn from(settings: PlayerSettings) -> Player {
        Player {
            acceleration: settings.normal_speed.acceleration.clone(),
            max_velocity: settings.normal_speed.max_velocity.clone(),
            jump_data:    None,
            used_dash:    false,
            settings:     settings,
        }
    }
}
