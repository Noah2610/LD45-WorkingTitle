use super::component_prelude::*;

#[derive(Default)]
pub struct Background;

impl Component for Background {
    type Storage = NullStorage<Self>;
}
