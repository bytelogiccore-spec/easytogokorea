use reqwest::Client;
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

/// Model info for a single translation direction
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: &'static str,
    /// Primary repo (onnx-community preferred)
    pub repo_id: &'static str,
    /// Files in onnx/ subdirectory
    pub onnx_files: &'static [&'static str],
    /// Files at repo root
    pub root_files: &'static [&'static str],
}

/// All supported translation models — using onnx-community repos with pre-exported ONNX
pub const MODELS: &[ModelInfo] = &[
    ModelInfo {
        name: "ko-en",
        repo_id: "onnx-community/opus-mt-ko-en",
        onnx_files: &["encoder_model.onnx", "decoder_model_merged.onnx"],
        root_files: &["tokenizer.json", "source.spm", "target.spm"],
    },
    ModelInfo {
        name: "en-zh",
        repo_id: "onnx-community/opus-mt-en-zh",
        onnx_files: &["encoder_model.onnx", "decoder_model_merged.onnx"],
        root_files: &["tokenizer.json", "source.spm", "target.spm"],
    },
    ModelInfo {
        name: "en-fr",
        repo_id: "onnx-community/opus-mt-en-fr",
        onnx_files: &["encoder_model.onnx", "decoder_model_merged.onnx"],
        root_files: &["tokenizer.json", "source.spm", "target.spm"],
    },
    ModelInfo {
        name: "en-de",
        repo_id: "onnx-community/opus-mt-en-de",
        onnx_files: &["encoder_model.onnx", "decoder_model_merged.onnx"],
        root_files: &["tokenizer.json", "source.spm", "target.spm"],
    },
    ModelInfo {
        name: "en-ru",
        repo_id: "onnx-community/opus-mt-en-ru",
        onnx_files: &["encoder_model.onnx", "decoder_model_merged.onnx"],
        root_files: &["tokenizer.json", "source.spm", "target.spm"],
    },
    ModelInfo {
        name: "en-ar",
        repo_id: "onnx-community/opus-mt-en-ar",
        onnx_files: &["encoder_model.onnx", "decoder_model_merged.onnx"],
        root_files: &["tokenizer.json", "source.spm", "target.spm"],
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

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(600))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    // Download ONNX files from onnx/ subdirectory
    for file_name in model.onnx_files {
        let file_path = model_dir.join(file_name);
        if file_path.exists() {
            continue;
        }

        let url = format!(
            "https://huggingface.co/{}/resolve/main/onnx/{}",
            model.repo_id, file_name
        );

        download_file(&client, &url, &file_path, model.name, progress)
            .await
            .map_err(|e| format!("Failed to download {file_name} for {}: {e}", model.name))?;
    }

    // Download root files (tokenizer, spm)
    for file_name in model.root_files {
        let file_path = model_dir.join(file_name);
        if file_path.exists() {
            continue;
        }

        let url = format!(
            "https://huggingface.co/{}/resolve/main/{}",
            model.repo_id, file_name
        );

        match download_file(&client, &url, &file_path, model.name, progress).await {
            Ok(_) => {}
            Err(e) => {
                // SPM files are nice to have but not critical if tokenizer.json exists
                if file_name.ends_with(".spm") {
                    eprintln!("Warning: could not download {file_name}: {e}");
                    continue;
                }
                return Err(format!("Failed to download {file_name} for {}: {e}", model.name));
            }
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
        .header("User-Agent", "EasyToGo/0.1.0")
        .send()
        .await
        .map_err(|e| format!("HTTP error: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("HTTP {} for {}", response.status(), url));
    }

    let total = response.content_length().unwrap_or(0);
    let mut stream = response.bytes_stream();

    // Write to temp file first, then rename (atomic download)
    let tmp_path = dest.with_extension("tmp");
    let mut file = tokio::fs::File::create(&tmp_path)
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
    drop(file);

    // Atomic rename
    tokio::fs::rename(&tmp_path, dest)
        .await
        .map_err(|e| format!("Rename error: {e}"))?;

    Ok(())
}
