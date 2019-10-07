pub fn resource<S>(path: S) -> String
where
    S: ToString,
{
    use amethyst::utils::app_root_dir::application_dir;

    let path = if cfg!(target = "windows") {
        path.to_string().replace("/", "\\")
    } else {
        path.to_string()
    };

    let res_dir =
        application_dir("resources").expect("Should have resources directory");

    let path = res_dir.join(path);
    path.to_str().unwrap().to_string()
}
