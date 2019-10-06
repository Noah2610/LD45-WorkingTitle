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

impl From<&str> for FeatureType {
    #[rustfmt::skip]
    fn from(s: &str) -> Self {
        match s {
            "AddCollisions"     => FeatureType::AddCollisions,
            "AddGravity1"       => FeatureType::AddGravity1,
            "AddJump"           => FeatureType::AddJump,
            "AddSingleSprite"   => FeatureType::AddSingleSprite,
            "AddAnimatedSprite" => FeatureType::AddAnimatedSprite,
            "SetSong1"          => FeatureType::SetSong1,
            "SetSong2"          => FeatureType::SetSong2,
            s                   => panic!(format!("Unknown feature_type {}", s)),
        }
    }
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
