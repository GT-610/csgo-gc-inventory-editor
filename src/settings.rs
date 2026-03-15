use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const SETTINGS_FILE: &str = "csgo_gc/editor/settings.json";

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub language: String,
    pub theme: Theme,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en-US".to_string(),
            theme: Theme::default(),
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, String> {
        if !PathBuf::from(SETTINGS_FILE).exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(SETTINGS_FILE)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse settings: {}", e))
    }

    pub fn save(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(SETTINGS_FILE, content)
            .map_err(|e| format!("Failed to write settings file: {}", e))?;

        Ok(())
    }

    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
}
