use super::component_prelude::*;

pub enum FeatureType {
    AddCollisions,
    AddGravity1,
    AddJump,
    AddSingleSprite,
    AddAnimatedSprite,
    SetSong1,
    SetSong2,
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
