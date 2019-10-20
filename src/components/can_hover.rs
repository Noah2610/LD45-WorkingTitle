use super::component_prelude::*;

#[derive(Default)]
pub struct CanHover;

impl Component for CanHover {
    type Storage = NullStorage<Self>;
}
