use super::component_prelude::*;

#[derive(Default)]
pub struct CanDash;

impl Component for CanDash {
    type Storage = NullStorage<Self>;
}
