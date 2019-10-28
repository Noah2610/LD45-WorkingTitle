use super::component_prelude::*;

#[derive(Default)]
pub struct Indicator;

impl Component for Indicator {
    type Storage = NullStorage<Self>;
}
