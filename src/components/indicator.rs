use super::component_prelude::*;

pub struct Indicator {
    pub feature_trigger: FeatureType,
}

impl Indicator {
    pub fn new(feature_trigger: FeatureType) -> Self {
        Self { feature_trigger }
    }
}

impl Component for Indicator {
    type Storage = VecStorage<Self>;
}
