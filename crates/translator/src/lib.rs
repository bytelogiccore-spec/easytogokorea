pub mod config;
pub mod downloader;
pub mod engine;       // NLLB-200 engine
pub mod engine_opus;  // Opus-MT engine (for mobile App use)

use std::collections::HashMap;
use std::sync::Mutex;

pub use downloader::{
    download_nllb, download_opus_models,
    is_nllb_downloaded, is_opus_model_downloaded,
    nllb_models_dir, base_models_dir,
    ProgressCallback,
};
pub use config::TranslatorConfig;
pub use engine::{NllbModel, lang_code_to_nllb};
pub use engine_opus::OpusMtModel;

/// Supported language display labels
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

/// Partner Desktop Translator — uses NLLB-200 exclusively
/// (Opus-MT engine is available in the crate for mobile app use)
pub struct Translator {
    nllb_model: Mutex<Option<NllbModel>>,
}

impl Translator {
    pub fn new() -> Self {
        eprintln!("[Translator] Engine: NLLB-200 (direct multilingual)");
        Self {
            nllb_model: Mutex::new(None),
        }
    }

    /// Preload the NLLB model at startup
    pub fn preload(&self) {
        eprintln!("[Translator] Preloading NLLB-200...");
        match self.ensure_nllb() {
            Ok(()) => eprintln!("[Translator] NLLB-200 preloaded successfully"),
            Err(e) => eprintln!("[Translator] Preload failed: {e}"),
        }
    }

    /// Translate text using NLLB-200
    pub fn translate(&self, text: &str, source: &str, target: &str) -> Result<String, String> {
        if source == target { return Ok(text.to_string()); }
        if text.trim().is_empty() { return Ok(String::new()); }

        self.ensure_nllb()?;
        let mut model = self.nllb_model.lock().map_err(|e| format!("Lock: {e}"))?;
        let m = model.as_mut().ok_or("NLLB not loaded")?;
        m.translate(text, source, target)
    }

    /// Translate to all supported languages at once
    pub fn translate_all(&self, text: &str, source: &str) -> HashMap<String, String> {
        let mut results = HashMap::new();
        for &(lang, _) in SUPPORTED_LANGUAGES {
            if lang == source { continue; }
            match self.translate(text, source, lang) {
                Ok(t) => { results.insert(lang.to_string(), t); }
                Err(e) => { results.insert(lang.to_string(), format!("[Error: {e}]")); }
            }
        }
        results
    }

    /// Check if NLLB model is downloaded
    pub fn is_model_ready(&self) -> bool {
        is_nllb_downloaded()
    }

    fn ensure_nllb(&self) -> Result<(), String> {
        let mut model = self.nllb_model.lock().map_err(|e| format!("Lock error: {e}"))?;
        if model.is_some() { return Ok(()); }

        if !is_nllb_downloaded() {
            return Err("NLLB 모델이 설치되지 않았습니다. 설정에서 다운로드해주세요.".to_string());
        }

        let dir = nllb_models_dir();
        let m = std::thread::Builder::new()
            .name("nllb-load".to_string())
            .stack_size(8 * 1024 * 1024)
            .spawn(move || NllbModel::load(&dir))
            .map_err(|e| format!("Thread spawn error: {e}"))?
            .join()
            .map_err(|_| "NLLB load thread panicked".to_string())??;
        *model = Some(m);
        Ok(())
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
        assert_eq!(lang_code_to_nllb("xx"), None);
    }
}
