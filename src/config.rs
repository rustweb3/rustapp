use {
    serde::Deserialize,
    std::{fs, path::PathBuf},
};

#[derive(Debug, Deserialize)]
pub struct MyConfig {
    pub main: Main,
}

const DEFAULT_CONFIG_PATH: &str = "./config.toml";

#[derive(Debug, Deserialize)]
pub struct Main {
    pub debug: bool,
}

pub fn home_file(sub_path: &str) -> Option<PathBuf> {
    match dirs::home_dir().unwrap() {
        user_home => Some(user_home.join(sub_path)),
    }
}

impl MyConfig {
    pub fn default_path() -> PathBuf {
        home_file(DEFAULT_CONFIG_PATH).unwrap()
    }
    pub fn load(path: &Option<PathBuf>, merge_env: bool) -> MyConfig {
        let config_path = {
            if path.is_none() {
                MyConfig::default_path()
            } else {
                path.as_ref().unwrap().to_path_buf()
            }
        };

        match fs::metadata(&config_path) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Config file not found: {:?}", config_path);
                std::process::exit(1);
            }
        }

        let config_content = fs::read_to_string(config_path).unwrap();
        let mut config: MyConfig = toml::from_str(&config_content).unwrap();

        if merge_env {
            config.merge_env();
        }
        config
    }
    pub fn merge_env(&mut self) {
        if let Ok(debug) = std::env::var("app_debug") {
            self.main.debug = debug.parse().unwrap();
        }
    }
}
