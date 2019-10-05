use super::component_prelude::*;

pub enum FeatureType {
    AddCollisions,
    AddGravity,
    AddJump,
}

pub struct Feature {
    pub applied:      bool,
    pub feature_type: FeatureType,
}

impl Feature {
    pub fn new(feature_type: FeatureType) -> Self {
        Self {
            applied:      false,
            feature_type: feature_type,
        }
    }
}

impl Component for Feature {
    type Storage = VecStorage<Self>;
}
