use super::component_prelude::*;

#[derive(Default)]
pub struct Tile;

impl Component for Tile {
    type Storage = NullStorage<Self>;
}
