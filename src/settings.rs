use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn get_settings_file() -> PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    exe_dir.join("csgo_gc").join("editor").join("settings.json")
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum MirrorSite {
    #[default]
    None,
    GhLlkk,
    CdnGhProxy,
    GhProxyCom,
    GhfastTop,
}

impl MirrorSite {
    pub fn get_prefix(&self) -> &'static str {
        match self {
            MirrorSite::None => "",
            MirrorSite::GhLlkk => "https://gh.llkk.cc/",
            MirrorSite::CdnGhProxy => "https://cdn.gh-proxy.org/",
            MirrorSite::GhProxyCom => "https://gh-proxy.com/",
            MirrorSite::GhfastTop => "https://ghfast.top/",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            MirrorSite::None => "None (Direct)",
            MirrorSite::GhLlkk => "gh.llkk.cc",
            MirrorSite::CdnGhProxy => "cdn.gh-proxy.org",
            MirrorSite::GhProxyCom => "gh-proxy.com",
            MirrorSite::GhfastTop => "ghfast.top",
        }
    }

    pub fn all() -> &'static [MirrorSite] {
        &[
            MirrorSite::None,
            MirrorSite::GhLlkk,
            MirrorSite::CdnGhProxy,
            MirrorSite::GhProxyCom,
            MirrorSite::GhfastTop,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub language: String,
    pub theme: Theme,
    #[serde(default)]
    pub use_online_metadata: bool,
    #[serde(default)]
    pub mirror_site: MirrorSite,
    #[serde(default)]
    pub last_online_update: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en-US".to_string(),
            theme: Theme::default(),
            use_online_metadata: false,
            mirror_site: MirrorSite::default(),
            last_online_update: None,
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, String> {
        let settings_file = get_settings_file();
        if !settings_file.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&settings_file)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse settings: {}", e))
    }

    pub fn save(&self) -> Result<(), String> {
        let settings_file = get_settings_file();

        // Create parent directories if they don't exist
        if let Some(parent) = settings_file.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create settings directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        fs::write(&settings_file, content)
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
