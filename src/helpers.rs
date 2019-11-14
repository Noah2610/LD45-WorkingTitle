use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

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
