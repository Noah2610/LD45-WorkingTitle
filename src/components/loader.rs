use super::component_prelude::*;

pub struct Loader {
    pub loading_distance: Vector,
}

impl Loader {
    pub fn new(loading_distance: Vector) -> Self {
        Self { loading_distance }
    }
}

impl Component for Loader {
    type Storage = VecStorage<Self>;
}
