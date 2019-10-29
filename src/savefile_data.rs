use crate::resources::prelude::CheckpointData;

#[derive(Deserialize, Serialize)]
pub struct SavefileData {
    pub checkpoint: CheckpointData,
}
