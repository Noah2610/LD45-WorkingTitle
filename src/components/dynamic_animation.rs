use super::component_prelude::*;

#[derive(Default)]
pub struct DynamicAnimation;

impl Component for DynamicAnimation {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct DynamicAnimationTrigger;

impl Component for DynamicAnimationTrigger {
    type Storage = NullStorage<Self>;
}
