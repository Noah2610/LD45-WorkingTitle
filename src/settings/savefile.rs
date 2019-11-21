use std::path::PathBuf;

use crate::helpers::{data_dir, file};

#[derive(Clone, Deserialize)]
pub struct SavefileSettings {
    pub filename: String,
}

impl SavefileSettings {
    pub fn path(&self) -> PathBuf {
        if let Some(mut path) = data_dir() {
            path.push(&self.filename);
            path
        } else {
            // Fallback savefile location is "./savefile.json"
            file(&self.filename).into()
        }
    }
}
