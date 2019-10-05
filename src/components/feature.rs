use super::component_prelude::*;

pub enum Feature {
    AddCollisions,
}

impl Component for Feature {
    type Storage = VecStorage<Self>;
}
