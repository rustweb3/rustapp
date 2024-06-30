use {
    crate::{cli, utils},
    anyhow::{Ok, Result},
    serde::{Deserialize, Serialize},
    std::{fs, path::PathBuf},
};

pub const APP_NAME: &str = "rustapp_template";

#[derive(Debug, Deserialize, Serialize)]
pub struct MyConfig {
    pub main: Main,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Main {
    pub debug: bool,
}

impl MyConfig {
    pub fn default() -> Self {
        Self {
            main: Main { debug: false },
        }
    }

    pub fn from_cli(cli: &cli::Cli) -> Self {
        let config_path = {
            if cli.config.is_some() {
                let cli_path: Option<PathBuf> = cli.config.clone();
                cli_path.unwrap()
            } else {
                let default_path = utils::default_app_config_path().unwrap();
                default_path
            }
        };
        MyConfig::load(&config_path).unwrap()
    }

    pub fn load(config_path: &PathBuf) -> Result<MyConfig> {
        let config_content = fs::read_to_string(config_path)?;
        let config: MyConfig = toml::from_str(&config_content)?;
        Ok(config)
    }
    pub fn merge_env(&mut self) {
        let debug = std::env::var("app_debug").unwrap_or_default();
        self.main.debug = debug.parse().unwrap_or_default();
    }
    pub fn merge_cli(&mut self, cli: &cli::Cli) {
        self.main.debug = cli.debug;
    }
}
