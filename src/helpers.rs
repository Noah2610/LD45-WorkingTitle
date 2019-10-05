pub fn resource<S>(path: S) -> String
where
    S: ToString,
{
    use amethyst::utils::app_root_dir::application_dir;
    let res_dir =
        application_dir("resources").expect("Should have resources directory");

    let path = res_dir.join(path.to_string());
    path.to_str().unwrap().to_string()
}
