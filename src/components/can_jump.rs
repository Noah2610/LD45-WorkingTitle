use super::component_prelude::*;

#[derive(Default)]
pub struct CanJump;

impl Component for CanJump {
    type Storage = NullStorage<Self>;
}
