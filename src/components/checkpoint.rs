use super::component_prelude::*;

#[derive(Default)]
pub struct Checkpoint {
    pub applied: bool,
}

impl Component for Checkpoint {
    type Storage = VecStorage<Self>;
}
