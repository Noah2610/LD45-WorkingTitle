use super::component_prelude::*;

#[derive(Default)]
pub struct HasAnimatedSprite;

impl Component for HasAnimatedSprite {
    type Storage = NullStorage<Self>;
}
