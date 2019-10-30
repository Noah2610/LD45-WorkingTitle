use std::cmp;

use super::component_prelude::*;

const FEATURE_TYPE_ORDER: [FeatureType; 9] = [
    FeatureType::AddCollisions,
    FeatureType::AddJump,
    FeatureType::AddSingleSprite,
    FeatureType::AddAnimatedSprite,
    FeatureType::AddEnemySprite,
    FeatureType::AddRun,
    FeatureType::AddDash,
    FeatureType::AddGravity1,
    FeatureType::AddGravity2,
];

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
        let self_pos = FEATURE_TYPE_ORDER.iter().position(|f| f == self);
        let other_pos = FEATURE_TYPE_ORDER.iter().position(|f| f == other);
        if let (Some(self_pos), Some(other_pos)) = (self_pos, other_pos) {
            self_pos.partial_cmp(&other_pos)
        } else {
            match (self, other) {
                (
                    FeatureType::SetSong(n_self),
                    FeatureType::SetSong(n_other),
                ) => n_self.partial_cmp(n_other),
                (FeatureType::SetSong(_), _) => Some(cmp::Ordering::Greater),
                (_, FeatureType::SetSong(_)) => Some(cmp::Ordering::Less),
                _ => Some(cmp::Ordering::Equal),
            }
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
