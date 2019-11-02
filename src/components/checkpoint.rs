use super::component_prelude::*;

pub type CheckpointId = usize;

pub struct Checkpoint {
    pub applied: bool,
    pub id:      CheckpointId,
}

impl Checkpoint {
    pub fn new(id: CheckpointId) -> Self {
        Self { id, applied: false }
    }
}

impl Component for Checkpoint {
    type Storage = VecStorage<Self>;
}
