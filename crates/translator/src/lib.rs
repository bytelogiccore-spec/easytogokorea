pub mod downloader;
pub mod engine;

use std::sync::Mutex;

pub use downloader::{download_model, is_model_downloaded, models_dir, ProgressCallback};
pub use engine::{NllbModel, lang_code_to_nllb};

/// Supported language codes (expandable — NLLB supports 200+)
pub const SUPPORTED_LANGUAGES: &[(&str, &str)] = &[
    ("ko", "한국어"),
    ("en", "English"),
    ("zh", "中文"),
    ("ja", "日本語"),
    ("fr", "Français"),
    ("de", "Deutsch"),
    ("es", "Español"),
    ("ru", "Русский"),
    ("ar", "العربية"),
    ("vi", "Tiếng Việt"),
    ("th", "ไทย"),
    ("id", "Bahasa"),
    ("pt", "Português"),
    ("it", "Italiano"),
    ("tr", "Türkçe"),
];

/// High-level translator (single NLLB model for all language pairs)
pub struct Translator {
    model: Mutex<Option<NllbModel>>,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            model: Mutex::new(None),
        }
    }

    /// Load the NLLB model if not already loaded
    fn ensure_model(&self) -> Result<(), String> {
        let mut model = self.model.lock().map_err(|e| format!("Lock error: {e}"))?;
        if model.is_some() {
            return Ok(());
        }

        if !is_model_downloaded() {
            return Err("NLLB model not downloaded. Please download from Settings.".to_string());
        }

        let m = NllbModel::load(&models_dir())?;
        *model = Some(m);
        Ok(())
    }

    /// Translate text from source language to target language
    pub fn translate(&self, text: &str, source: &str, target: &str) -> Result<String, String> {
        if source == target {
            return Ok(text.to_string());
        }
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        self.ensure_model()?;
        let mut model = self.model.lock().map_err(|e| format!("Lock error: {e}"))?;
        let m = model.as_mut().ok_or("Model not loaded")?;
        m.translate(text, source, target)
    }

    /// Translate text to all supported languages
    pub fn translate_all(&self, text: &str, source: &str) -> std::collections::HashMap<String, String> {
        let mut results = std::collections::HashMap::new();
        for &(lang, _) in SUPPORTED_LANGUAGES {
            if lang == source { continue; }
            match self.translate(text, source, lang) {
                Ok(t) => { results.insert(lang.to_string(), t); }
                Err(e) => { results.insert(lang.to_string(), format!("[Error: {e}]")); }
            }
        }
        results
    }
}

impl Default for Translator {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nllb_lang_codes() {
        assert_eq!(lang_code_to_nllb("ko"), Some("kor_Hang"));
        assert_eq!(lang_code_to_nllb("en"), Some("eng_Latn"));
        assert_eq!(lang_code_to_nllb("zh"), Some("zho_Hans"));
        assert_eq!(lang_code_to_nllb("xx"), None);
    }

    #[test]
    fn test_supported_languages() {
        assert!(SUPPORTED_LANGUAGES.len() >= 15);
    }
}
