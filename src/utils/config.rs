use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    #[serde(default)]
    pub first_run: bool,
    #[serde(default)]
    pub player: PlayerConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub network: NetworkConfig,
    #[serde(default)]
    pub auth: AuthConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct UiConfig {
    pub is_dark_theme: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct PlayerConfig {
    pub fullscreen_on_play: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct NetworkConfig {
    pub proxy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct AuthConfig {
    pub token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            player: PlayerConfig {
                fullscreen_on_play: false,
            },
            first_run: true,
            ui: UiConfig {
                is_dark_theme: false,
            },
            network: NetworkConfig {
                proxy: Some(String::new()),
            },
            auth: AuthConfig {
                token: Some(String::new()),
            },
        }
    }
}

impl Config {
    pub fn load() -> Self {
        match load_config() {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error reading config: {}. Using default", e);
                Config::default()
            }
        }
    }

    pub fn save(&self) {
        println!("Saving config");
        let path = get_state_path();
        let output = toml::to_string(&self).unwrap();
        fs::write(path, output).expect("Failed to write state file");
    }

    pub fn update_token(&mut self, token: String) {
        self.auth.token = Some(token);
        self.first_run = false;
        self.save();
    }

    pub fn skip_login(&mut self) {
        self.auth.token = None;
        self.first_run = false;
        self.save();
    }
}

fn get_state_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("kz", "findmyname284", "anixart") {
        let config_dir = proj_dirs.config_dir();
        std::fs::create_dir_all(config_dir).expect("Failed to create config directory");
        return config_dir.join(CONFIG_FILE);
    }
    PathBuf::from(CONFIG_FILE)
}

fn load_config() -> Result<Config, Error> {
    let config_dir = get_state_path();

    if !config_dir.exists() {
        return Ok(Config::default());
    }

    let contents = std::fs::read_to_string(&config_dir)?;

    if contents.is_empty() {
        println!("Config file is empty");
        return Ok(Config::default());
    }

    let config: Config = toml::from_str(&contents).expect("Failed to parse config");

    Ok(config)
}
