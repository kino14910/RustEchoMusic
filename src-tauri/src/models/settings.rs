use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub theme: ThemeMode,
    pub volume: u8,
    pub library_dirs: Vec<String>,
    pub scan_on_startup: bool,
    pub reduce_motion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ThemeMode {
    Auto,
    Light,
    Dark,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Auto,
            volume: 80,
            library_dirs: Vec::new(),
            scan_on_startup: false,
            reduce_motion: false,
        }
    }
}