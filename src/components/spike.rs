use super::component_prelude::*;

pub struct Spike {
    pub enabled: bool,
}

impl Default for Spike {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Component for Spike {
    type Storage = VecStorage<Self>;
}
