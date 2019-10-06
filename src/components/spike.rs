use super::component_prelude::*;

#[derive(Default)]
pub struct Spike;

impl Component for Spike {
    type Storage = NullStorage<Self>;
}
