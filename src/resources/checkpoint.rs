use crate::components::prelude::FeatureType;
use deathframe::geo::Vector;

#[derive(Clone)]
pub struct CheckpointData {
    pub position: Vector,
    pub features: Vec<FeatureType>,
}

#[derive(Default)]
pub struct CheckpointRes(pub Option<CheckpointData>);
