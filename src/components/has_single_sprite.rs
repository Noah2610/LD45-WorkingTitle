use super::component_prelude::*;

#[derive(Default)]
pub struct HasSingleSprite;

impl Component for HasSingleSprite {
    type Storage = NullStorage<Self>;
}
