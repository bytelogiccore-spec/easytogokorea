pub mod downloader;
pub mod engine;

use std::collections::HashMap;
use std::sync::Mutex;

pub use downloader::{download_model, download_all_models, is_model_downloaded, models_dir, MODELS, ProgressCallback};
pub use engine::TranslationModel;

/// Supported language codes
pub const SUPPORTED_LANGUAGES: &[(&str, &str)] = &[
    ("ko", "한국어"),
    ("en", "English"),
    ("zh", "中文"),
    ("fr", "Français"),
    ("de", "Deutsch"),
    ("ru", "Русский"),
    ("ar", "العربية"),
];

/// Get the model name needed to translate from source to target
pub fn get_model_name(source: &str, target: &str) -> Option<String> {
    if source == target {
        return None;
    }
    // Direct ko<->en
    if source == "ko" && target == "en" {
        return Some("ko-en".to_string());
    }
    if source == "en" && target == "ko" {
        return Some("en-ko".to_string());
    }
    // en -> other
    if source == "en" {
        return Some(format!("en-{target}"));
    }
    // ko -> other (needs 2-step: ko->en->target)
    // Return the first step model name
    if source == "ko" {
        return Some("ko-en".to_string());
    }
    None
}

/// High-level translator that manages loaded models
pub struct Translator {
    models: Mutex<HashMap<String, TranslationModel>>,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            models: Mutex::new(HashMap::new()),
        }
    }

    /// Load a model if not already loaded
    fn ensure_model(&self, model_name: &str) -> Result<(), String> {
        let mut models = self.models.lock().map_err(|e| format!("Lock error: {e}"))?;
        if models.contains_key(model_name) {
            return Ok(());
        }

        let model_dir = models_dir().join(model_name);
        if !model_dir.exists() {
            return Err(format!("Model {model_name} not downloaded. Call download first."));
        }

        let model = TranslationModel::load(&model_dir, model_name)?;
        models.insert(model_name.to_string(), model);
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

        // Direct translation (ko<->en, en->X)
        if source == "en" || (source == "ko" && target == "en") || (source == "en" && target == "ko") {
            let model_name = get_model_name(source, target)
                .ok_or_else(|| format!("No model for {source}->{target}"))?;
            self.ensure_model(&model_name)?;
            let mut models = self.models.lock().map_err(|e| format!("Lock error: {e}"))?;
            let model = models.get_mut(&model_name)
                .ok_or_else(|| format!("Model {model_name} not loaded"))?;
            return model.translate(text);
        }

        // 2-step translation: ko -> en -> target
        if source == "ko" {
            // Step 1: ko -> en
            self.ensure_model("ko-en")?;
            let english = {
                let mut models = self.models.lock().map_err(|e| format!("Lock error: {e}"))?;
                let model = models.get_mut("ko-en").ok_or("ko-en model not loaded")?;
                model.translate(text)?
            };

            // Step 2: en -> target
            let target_model = format!("en-{target}");
            self.ensure_model(&target_model)?;
            let mut models = self.models.lock().map_err(|e| format!("Lock error: {e}"))?;
            let model = models.get_mut(&target_model)
                .ok_or_else(|| format!("Model {target_model} not loaded"))?;
            return model.translate(&english);
        }

        Err(format!("Unsupported translation: {source} -> {target}"))
    }

    /// Translate text to all supported languages at once
    pub fn translate_all(&self, text: &str, source: &str) -> HashMap<String, String> {
        let mut results = HashMap::new();
        for &(lang, _) in SUPPORTED_LANGUAGES {
            if lang == source {
                continue;
            }
            match self.translate(text, source, lang) {
                Ok(translated) => { results.insert(lang.to_string(), translated); }
                Err(e) => { results.insert(lang.to_string(), format!("[Error: {e}]")); }
            }
        }
        results
    }
}

impl Default for Translator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_name() {
        assert_eq!(get_model_name("ko", "en"), Some("ko-en".to_string()));
        assert_eq!(get_model_name("en", "fr"), Some("en-fr".to_string()));
        assert_eq!(get_model_name("ko", "fr"), Some("ko-en".to_string())); // first step
        assert_eq!(get_model_name("ko", "ko"), None);
    }

    #[test]
    fn test_supported_languages() {
        assert_eq!(SUPPORTED_LANGUAGES.len(), 7);
    }
}
