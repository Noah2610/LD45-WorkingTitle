use std::fs::{create_dir_all, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use amethyst::utils::app_root_dir::application_root_dir;

pub fn file<S>(path: S) -> String
where
    S: ToString,
{
    use amethyst::utils::app_root_dir::application_dir;

    let path = if cfg!(target_os = "windows") {
        path.to_string().replace("/", "\\")
    } else {
        path.to_string()
    };

    application_dir(path).unwrap().to_str().unwrap().to_string()
}

pub fn resource<S>(path: S) -> String
where
    S: ToString,
{
    use amethyst::utils::app_root_dir::application_dir;

    let path = if cfg!(target_os = "windows") {
        path.to_string().replace("/", "\\")
    } else {
        path.to_string()
    };

    let res_dir =
        application_dir("resources").expect("Should have resources directory");

    let path = res_dir.join(path);
    path.to_str().unwrap().to_string()
}

pub fn write_file<P, S>(path: P, data: S) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    S: ToString,
{
    let mut file = File::create(path)?;
    write!(&mut file, "{}", data.to_string())
}

pub fn data_dir() -> Option<PathBuf> {
    if let Some(mut path) = dirs::data_local_dir() {
        path.push(crate::meta::NAME);
        if !path.is_dir() {
            create_dir_all(&path).unwrap();
        }
        Some(path)
    } else {
        application_root_dir().ok()
    }
}
