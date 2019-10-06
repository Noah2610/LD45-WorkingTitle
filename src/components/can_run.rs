use super::component_prelude::*;

#[derive(Default)]
pub struct CanRun;

impl Component for CanRun {
    type Storage = NullStorage<Self>;
}
