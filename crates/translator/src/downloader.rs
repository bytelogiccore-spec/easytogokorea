use reqwest::Client;
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

/// NLLB-200-distilled-600M ONNX model from HuggingFace
const REPO_ID: &str = "AxolsWebAI/nllb-200-distilled-600M-onnx";

/// Files to download from the onnx/ subdirectory (quantized for smaller size)
const ONNX_FILES: &[&str] = &[
    "encoder_model_quantized.onnx",       // ~416 MB
    "decoder_model_merged_quantized.onnx", // ~731 MB
];

/// Files to download from the repo root
const ROOT_FILES: &[&str] = &[
    "tokenizer.json",      // ~32 MB
    "config.json",
];

/// Progress callback type: (file_name, downloaded_bytes, total_bytes)
pub type ProgressCallback = Box<dyn Fn(&str, u64, u64) + Send + Sync>;

/// Get the models cache directory
pub fn models_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("easytogokorea")
        .join("models")
        .join("nllb-200")
}

/// Check if the NLLB model is already downloaded
pub fn is_model_downloaded() -> bool {
    let dir = models_dir();
    if !dir.exists() {
        return false;
    }
    // Check that quantized encoder exists
    dir.join("encoder_model_quantized.onnx").exists()
        && dir.join("decoder_model_merged_quantized.onnx").exists()
        && dir.join("tokenizer.json").exists()
}

/// Download the NLLB-200 model from HuggingFace Hub
pub async fn download_model(
    progress: Option<&ProgressCallback>,
) -> Result<PathBuf, String> {
    let model_dir = models_dir();
    tokio::fs::create_dir_all(&model_dir)
        .await
        .map_err(|e| format!("Failed to create model dir: {e}"))?;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(1800)) // 30 min for large files
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    // Download ONNX model files
    for file_name in ONNX_FILES {
        let file_path = model_dir.join(file_name);
        if file_path.exists() {
            continue;
        }

        let url = format!(
            "https://huggingface.co/{}/resolve/main/onnx/{}",
            REPO_ID, file_name
        );

        download_file(&client, &url, &file_path, file_name, progress)
            .await
            .map_err(|e| format!("Failed to download {file_name}: {e}"))?;
    }

    // Download root files (tokenizer, config)
    for file_name in ROOT_FILES {
        let file_path = model_dir.join(file_name);
        if file_path.exists() {
            continue;
        }

        let url = format!(
            "https://huggingface.co/{}/resolve/main/{}",
            REPO_ID, file_name
        );

        download_file(&client, &url, &file_path, file_name, progress)
            .await
            .map_err(|e| format!("Failed to download {file_name}: {e}"))?;
    }

    Ok(model_dir)
}

async fn download_file(
    client: &Client,
    url: &str,
    dest: &Path,
    file_label: &str,
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
            cb(file_label, downloaded, total);
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
