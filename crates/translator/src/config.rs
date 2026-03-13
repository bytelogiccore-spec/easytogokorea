use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE: &str = "translator_config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslatorConfig {
    /// Default engine: "OpusMT" or "Nllb200"
    pub default_engine: String,
}

impl Default for TranslatorConfig {
    fn default() -> Self {
        Self {
            default_engine: "OpusMT".to_string(),
        }
    }
}

impl TranslatorConfig {
    fn config_path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("easytogokorea")
            .join(CONFIG_FILE)
    }

    /// Load config from disk, or return default
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(json) => match serde_json::from_str(&json) {
                    Ok(cfg) => {
                        eprintln!("[Config] Loaded: {:?}", path);
                        return cfg;
                    }
                    Err(e) => eprintln!("[Config] Parse error, using default: {e}"),
                },
                Err(e) => eprintln!("[Config] Read error, using default: {e}"),
            }
        }
        eprintln!("[Config] Using defaults");
        Self::default()
    }

    /// Save config to disk
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Dir create error: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Serialize error: {e}"))?;
        std::fs::write(&path, json)
            .map_err(|e| format!("Write error: {e}"))?;
        eprintln!("[Config] Saved to {:?}", path);
        Ok(())
    }
}
