use super::component_prelude::*;

#[derive(Default)]
pub struct Goal;

impl Component for Goal {
    type Storage = NullStorage<Self>;
}
