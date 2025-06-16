use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

const STATE_FILE: &str = "app.json";

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppState {
    pub token: Option<String>,
    pub first_run: bool,
}

impl AppState {
    pub fn load() -> Self {
        let path = get_state_path();
        if !path.exists() {
            return AppState {
                token: None,
                first_run: true,
            };
        }

        match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| {
                eprintln!("Error parsing state file, using default");
                AppState::default()
            }),
            Err(_) => {
                eprintln!("Error reading state file, using default");
                AppState::default()
            }
        }
    }

    pub fn save(&self) {
        let path = get_state_path();
        let json = serde_json::to_string_pretty(self).expect("Failed to serialize state");
        fs::write(path, json).expect("Failed to write state file");
    }

    pub fn update_token(&mut self, token: String) {
        self.token = Some(token);
        self.first_run = false;
        self.save();
    }
    
    pub fn skip_login(&mut self) {
        self.token = None;
        self.first_run = false;
        self.save();
    }
}

fn get_state_path() -> PathBuf {
    PathBuf::from(STATE_FILE)
}