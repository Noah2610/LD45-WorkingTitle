use std::fs::create_dir_all;
use std::path::PathBuf;

use crate::helpers::file;

#[derive(Clone, Deserialize)]
pub struct SavefileSettings {
    pub filename: String,
}

impl SavefileSettings {
    pub fn path(&self) -> PathBuf {
        if let Some(mut path) = dirs::data_local_dir() {
            path.push(crate::meta::NAME);
            if !path.is_dir() {
                create_dir_all(&path).unwrap();
            }
            path.push(&self.filename);
            path
        } else {
            // Fallback savefile location is "./savefile.json"
            file(&self.filename).into()
        }
    }
}
