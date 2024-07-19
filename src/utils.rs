use {crate::config::APP_NAME, std::path::PathBuf};

pub fn app_dir(sub_path: &str) -> Option<PathBuf> {
    let sub_path = format!(".config/{}{}", APP_NAME, sub_path);
    let user_home = dirs::home_dir();
    if user_home.is_none() {
        None
    } else {
        Some(user_home.unwrap().join(sub_path))
    }
}

pub fn default_app_config_path() -> Option<PathBuf> {
    app_dir("/config/config.toml")
}
