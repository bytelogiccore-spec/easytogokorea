pub mod downloader;
pub mod engine;       // NLLB-200 engine
pub mod engine_opus;  // Opus-MT engine

use std::collections::HashMap;
use std::sync::Mutex;

pub use downloader::{
    download_nllb, download_opus_models,
    is_nllb_downloaded, is_opus_model_downloaded,
    nllb_models_dir, base_models_dir,
    ProgressCallback,
};
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

/// Opus-MT model pairs (onnx-community)
const OPUS_MODELS: &[(&str, &str)] = &[
    ("ko-en", "onnx-community/opus-mt-ko-en"),
    ("en-zh", "onnx-community/opus-mt-en-zh"),
    ("en-fr", "onnx-community/opus-mt-en-fr"),
    ("en-de", "onnx-community/opus-mt-en-de"),
    ("en-ru", "onnx-community/opus-mt-en-ru"),
    ("en-ar", "onnx-community/opus-mt-en-ar"),
];

/// Translation engine type
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EngineType {
    OpusMT,
    Nllb200,
}

/// Check which Opus-MT models are downloaded
pub fn opus_models_dir() -> std::path::PathBuf {
    base_models_dir()
}

pub fn is_any_opus_downloaded() -> bool {
    is_opus_model_downloaded("ko-en")
}

/// Get Opus-MT model name for a language pair
fn opus_model_name(source: &str, target: &str) -> Option<String> {
    if source == "ko" && target == "en" { return Some("ko-en".to_string()); }
    if source == "en" { return Some(format!("en-{target}")); }
    if source == "ko" { return Some("ko-en".to_string()); } // ko→X: first step ko→en
    None
}

/// High-level translator with dual engine support
pub struct Translator {
    engine: Mutex<EngineType>,
    nllb_model: Mutex<Option<NllbModel>>,
    opus_models: Mutex<HashMap<String, OpusMtModel>>,
}

impl Translator {
    pub fn new() -> Self {
        // Default to Opus-MT if models are available, else NLLB
        let default_engine = if is_any_opus_downloaded() {
            EngineType::OpusMT
        } else if is_nllb_downloaded() {
            EngineType::Nllb200
        } else {
            EngineType::OpusMT
        };
        eprintln!("[Translator] Default engine: {:?}", default_engine);

        Self {
            engine: Mutex::new(default_engine),
            nllb_model: Mutex::new(None),
            opus_models: Mutex::new(HashMap::new()),
        }
    }

    /// Get current engine type
    pub fn current_engine(&self) -> EngineType {
        self.engine.lock().unwrap().clone()
    }

    /// Switch engine
    pub fn set_engine(&self, engine: EngineType) {
        eprintln!("[Translator] Switching to {:?}", engine);
        *self.engine.lock().unwrap() = engine;
    }

    /// Translate with the selected engine
    pub fn translate(&self, text: &str, source: &str, target: &str) -> Result<String, String> {
        if source == target { return Ok(text.to_string()); }
        if text.trim().is_empty() { return Ok(String::new()); }

        let engine = self.engine.lock().map_err(|e| format!("Lock error: {e}"))?.clone();

        match engine {
            EngineType::OpusMT => self.translate_opus(text, source, target),
            EngineType::Nllb200 => self.translate_nllb(text, source, target),
        }
    }

    // ─── Opus-MT translation ───

    fn ensure_opus_model(&self, model_name: &str) -> Result<(), String> {
        let mut models = self.opus_models.lock().map_err(|e| format!("Lock error: {e}"))?;
        if models.contains_key(model_name) { return Ok(()); }

        let model_dir = opus_models_dir().join(model_name);
        if !model_dir.exists() { return Err(format!("Opus-MT model '{}' not downloaded", model_name)); }

        // Load model on a thread with 8MB stack (ONNX graph parsing needs large stack)
        let dir = model_dir.clone();
        let model = std::thread::Builder::new()
            .name(format!("opus-load-{model_name}"))
            .stack_size(8 * 1024 * 1024) // 8MB stack
            .spawn(move || OpusMtModel::load(&dir))
            .map_err(|e| format!("Thread spawn error: {e}"))?
            .join()
            .map_err(|_| "Model load thread panicked".to_string())??;

        models.insert(model_name.to_string(), model);
        Ok(())
    }

    fn translate_opus(&self, text: &str, source: &str, target: &str) -> Result<String, String> {
        // Direct: ko→en, en→X
        if source == "en" || (source == "ko" && target == "en") {
            let model_name = opus_model_name(source, target)
                .ok_or_else(|| format!("No Opus-MT model for {source}→{target}"))?;
            self.ensure_opus_model(&model_name)?;
            let mut models = self.opus_models.lock().map_err(|e| format!("Lock: {e}"))?;
            let m = models.get_mut(&model_name).ok_or("Model not loaded")?;
            return m.translate(text);
        }

        // ko → other: 2-step (ko→en, then en→target)
        if source == "ko" {
            self.ensure_opus_model("ko-en")?;
            let english = {
                let mut models = self.opus_models.lock().map_err(|e| format!("Lock: {e}"))?;
                let m = models.get_mut("ko-en").ok_or("ko-en not loaded")?;
                m.translate(text)?
            };
            let target_model = format!("en-{target}");
            self.ensure_opus_model(&target_model)?;
            let mut models = self.opus_models.lock().map_err(|e| format!("Lock: {e}"))?;
            let m = models.get_mut(&target_model).ok_or("Target model not loaded")?;
            return m.translate(&english);
        }

        Err(format!("Unsupported Opus-MT: {source}→{target}"))
    }

    // ─── NLLB-200 translation ───

    fn ensure_nllb(&self) -> Result<(), String> {
        let mut model = self.nllb_model.lock().map_err(|e| format!("Lock error: {e}"))?;
        if model.is_some() { return Ok(()); }

        if !is_nllb_downloaded() { return Err("NLLB model not downloaded".to_string()); }

        // Load on thread with 8MB stack
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

    fn translate_nllb(&self, text: &str, source: &str, target: &str) -> Result<String, String> {
        self.ensure_nllb()?;
        let mut model = self.nllb_model.lock().map_err(|e| format!("Lock: {e}"))?;
        let m = model.as_mut().ok_or("NLLB not loaded")?;
        m.translate(text, source, target)
    }

    /// Translate to all languages
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

    /// Get which models are available
    pub fn available_engines(&self) -> HashMap<String, bool> {
        let mut status = HashMap::new();
        status.insert("opus-mt".to_string(), is_opus_model_downloaded("ko-en"));
        status.insert("nllb-200".to_string(), is_nllb_downloaded());
        status
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

    #[test]
    fn test_opus_model_name() {
        assert_eq!(opus_model_name("ko", "en"), Some("ko-en".to_string()));
        assert_eq!(opus_model_name("en", "fr"), Some("en-fr".to_string()));
        assert_eq!(opus_model_name("ko", "fr"), Some("ko-en".to_string()));
    }
}
