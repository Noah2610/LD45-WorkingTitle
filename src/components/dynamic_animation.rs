use super::component_prelude::*;

#[derive(Default)]
pub struct DynamicAnimation;

impl Component for DynamicAnimation {
    type Storage = NullStorage<Self>;
}
