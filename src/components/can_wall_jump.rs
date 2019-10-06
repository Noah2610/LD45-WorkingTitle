use super::component_prelude::*;

#[derive(Default)]
pub struct CanWallJump;

impl Component for CanWallJump {
    type Storage = NullStorage<Self>;
}
