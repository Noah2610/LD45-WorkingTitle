use std::cmp;

use super::component_prelude::*;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum FeatureType {
    AddCollisions,
    AddGravity1,
    AddGravity2,
    AddJump,
    AddSingleSprite,
    AddAnimatedSprite,
    AddEnemySprite,
    AddRun,
    AddDash,
    SetSong(usize),
}

impl From<&str> for FeatureType {
    #[rustfmt::skip]
    fn from(s: &str) -> Self {
        match s {
            "AddCollisions"     => FeatureType::AddCollisions,
            "AddGravity1"       => FeatureType::AddGravity1,
            "AddGravity2"       => FeatureType::AddGravity2,
            "AddJump"           => FeatureType::AddJump,
            "AddSingleSprite"   => FeatureType::AddSingleSprite,
            "AddAnimatedSprite" => FeatureType::AddAnimatedSprite,
            "AddEnemySprite"    => FeatureType::AddEnemySprite,
            "AddRun"            => FeatureType::AddRun,
            "AddDash"           => FeatureType::AddDash,
            s if s.starts_with("SetSong") => {
                FeatureType::SetSong((&s[7 ..]).parse::<usize>().expect(
                    "Characters after 'SetSong' can only be integers, for \
                     FeatureType::SetSong",
                ))
            }
            s => panic!(format!("Unknown feature_type {}", s)),
        }
    }
}

impl cmp::PartialOrd for FeatureType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (FeatureType::AddGravity1, FeatureType::AddGravity2) => {
                Some(cmp::Ordering::Less)
            }
            (FeatureType::AddGravity2, FeatureType::AddGravity1) => {
                Some(cmp::Ordering::Greater)
            }
            (FeatureType::SetSong(n1), FeatureType::SetSong(n2)) => {
                n1.partial_cmp(n2)
            }
            (FeatureType::AddSingleSprite, FeatureType::AddAnimatedSprite) => {
                Some(cmp::Ordering::Less)
            }
            (FeatureType::AddAnimatedSprite, FeatureType::AddSingleSprite) => {
                Some(cmp::Ordering::Greater)
            }
            _ => Some(cmp::Ordering::Equal),
        }
    }
}

#[derive(Debug)]
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

#[derive(Default)]
pub struct ForceApplyFeature;

impl Component for ForceApplyFeature {
    type Storage = NullStorage<Self>;
}
