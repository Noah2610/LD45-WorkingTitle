use super::component_prelude::*;

pub struct Player {
    pub settings: PlayerSettings,
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}

impl From<PlayerSettings> for Player {
    fn from(settings: PlayerSettings) -> Player {
        Player { settings }
    }
}
