use crate::components::prelude::{CheckpointId, FeatureType};
use deathframe::geo::Vector;

#[derive(Clone, Deserialize, Serialize)]
pub struct CheckpointData {
    pub position:    Vector,
    pub features:    Vec<FeatureType>,
    pub checkpoints: Vec<CheckpointId>,
}

#[derive(Default)]
pub struct CheckpointRes(pub Option<CheckpointData>);
