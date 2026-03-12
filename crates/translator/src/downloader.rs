use reqwest::Client;
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

/// Model info for a single translation direction
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: &'static str,
    pub repo_id: &'static str,
    pub files: &'static [&'static str],
}

/// All supported translation models
pub const MODELS: &[ModelInfo] = &[
    ModelInfo {
        name: "ko-en",
        repo_id: "Helsinki-NLP/opus-mt-ko-en",
        files: &["encoder_model.onnx", "decoder_model.onnx", "source.spm", "target.spm", "tokenizer.json"],
    },
    ModelInfo {
        name: "en-ko",
        repo_id: "Helsinki-NLP/opus-mt-en-ko",
        files: &["encoder_model.onnx", "decoder_model.onnx", "source.spm", "target.spm", "tokenizer.json"],
    },
    ModelInfo {
        name: "en-zh",
        repo_id: "Helsinki-NLP/opus-mt-en-zh",
        files: &["encoder_model.onnx", "decoder_model.onnx", "source.spm", "target.spm", "tokenizer.json"],
    },
    ModelInfo {
        name: "en-fr",
        repo_id: "Helsinki-NLP/opus-mt-en-fr",
        files: &["encoder_model.onnx", "decoder_model.onnx", "source.spm", "target.spm", "tokenizer.json"],
    },
    ModelInfo {
        name: "en-de",
        repo_id: "Helsinki-NLP/opus-mt-en-de",
        files: &["encoder_model.onnx", "decoder_model.onnx", "source.spm", "target.spm", "tokenizer.json"],
    },
    ModelInfo {
        name: "en-ru",
        repo_id: "Helsinki-NLP/opus-mt-en-ru",
        files: &["encoder_model.onnx", "decoder_model.onnx", "source.spm", "target.spm", "tokenizer.json"],
    },
    ModelInfo {
        name: "en-ar",
        repo_id: "Helsinki-NLP/opus-mt-en-ar",
        files: &["encoder_model.onnx", "decoder_model.onnx", "source.spm", "target.spm", "tokenizer.json"],
    },
];

/// Progress callback type: (model_name, downloaded_bytes, total_bytes)
pub type ProgressCallback = Box<dyn Fn(&str, u64, u64) + Send + Sync>;

/// Get the models cache directory
pub fn models_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("easytogokorea")
        .join("models")
}

/// Check if a specific model is already downloaded
pub fn is_model_downloaded(model_name: &str) -> bool {
    let dir = models_dir().join(model_name);
    if !dir.exists() {
        return false;
    }
    // Check for encoder at minimum
    dir.join("encoder_model.onnx").exists()
}

/// Download a single model from HuggingFace Hub
pub async fn download_model(
    model: &ModelInfo,
    progress: Option<&ProgressCallback>,
) -> Result<PathBuf, String> {
    let model_dir = models_dir().join(model.name);
    tokio::fs::create_dir_all(&model_dir)
        .await
        .map_err(|e| format!("Failed to create model dir: {e}"))?;

    let client = Client::new();

    for file_name in model.files {
        let file_path = model_dir.join(file_name);
        if file_path.exists() {
            continue; // skip already downloaded files
        }

        // Try ONNX subdirectory first (optimum-exported), then root
        let urls = vec![
            format!(
                "https://huggingface.co/{}/resolve/main/onnx/{}",
                model.repo_id, file_name
            ),
            format!(
                "https://huggingface.co/{}/resolve/main/{}",
                model.repo_id, file_name
            ),
        ];

        let mut downloaded = false;
        for url in &urls {
            match download_file(&client, url, &file_path, model.name, progress).await {
                Ok(_) => {
                    downloaded = true;
                    break;
                }
                Err(_) => continue,
            }
        }

        if !downloaded {
            // Non-critical files (spm) can be skipped
            if !file_name.ends_with(".onnx") && !file_name.ends_with(".json") {
                continue;
            }
            return Err(format!("Failed to download {file_name} for {}", model.name));
        }
    }

    Ok(model_dir)
}

/// Download all required models with progress
pub async fn download_all_models(
    progress: Option<&ProgressCallback>,
) -> Result<(), String> {
    for model in MODELS {
        if !is_model_downloaded(model.name) {
            download_model(model, progress).await?;
        }
    }
    Ok(())
}

async fn download_file(
    client: &Client,
    url: &str,
    dest: &Path,
    model_name: &str,
    progress: Option<&ProgressCallback>,
) -> Result<(), String> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let total = response.content_length().unwrap_or(0);
    let mut stream = response.bytes_stream();
    let mut file = tokio::fs::File::create(dest)
        .await
        .map_err(|e| format!("File create error: {e}"))?;

    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Stream error: {e}"))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Write error: {e}"))?;
        downloaded += chunk.len() as u64;

        if let Some(cb) = &progress {
            cb(model_name, downloaded, total);
        }
    }

    file.flush().await.map_err(|e| format!("Flush error: {e}"))?;
    Ok(())
}
