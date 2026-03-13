use reqwest::Client;
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

/// Progress callback type: (file_name, downloaded_bytes, total_bytes)
pub type ProgressCallback = Box<dyn Fn(&str, u64, u64) + Send + Sync>;

// ─── NLLB-200 ───

const NLLB_REPO: &str = "AxolsWebAI/nllb-200-distilled-600M-onnx";

const NLLB_ONNX_FILES: &[&str] = &[
    "encoder_model_quantized.onnx",  // ~416 MB
    "decoder_model_quantized.onnx",  // ~731 MB (non-merged, no KV cache)
];

const NLLB_ROOT_FILES: &[&str] = &[
    "tokenizer.json",
    "config.json",
];

pub fn nllb_models_dir() -> PathBuf {
    base_models_dir().join("nllb-200")
}

pub fn is_nllb_downloaded() -> bool {
    let dir = nllb_models_dir();
    dir.join("encoder_model_quantized.onnx").exists()
        && dir.join("decoder_model_quantized.onnx").exists()
        && dir.join("tokenizer.json").exists()
}

pub async fn download_nllb(progress: Option<&ProgressCallback>) -> Result<PathBuf, String> {
    let model_dir = nllb_models_dir();
    tokio::fs::create_dir_all(&model_dir).await
        .map_err(|e| format!("Failed to create dir: {e}"))?;

    let client = make_client()?;

    for file_name in NLLB_ONNX_FILES {
        let file_path = model_dir.join(file_name);
        if file_path.exists() { continue; }
        let url = format!("https://huggingface.co/{}/resolve/main/onnx/{}", NLLB_REPO, file_name);
        download_file(&client, &url, &file_path, file_name, progress).await
            .map_err(|e| format!("Failed to download {file_name}: {e}"))?;
    }

    for file_name in NLLB_ROOT_FILES {
        let file_path = model_dir.join(file_name);
        if file_path.exists() { continue; }
        let url = format!("https://huggingface.co/{}/resolve/main/{}", NLLB_REPO, file_name);
        download_file(&client, &url, &file_path, file_name, progress).await
            .map_err(|e| format!("Failed to download {file_name}: {e}"))?;
    }

    Ok(model_dir)
}

// ─── Opus-MT ───

pub struct OpusModelInfo {
    pub name: &'static str,
    pub repo: &'static str,
}

pub const OPUS_MODELS: &[OpusModelInfo] = &[
    OpusModelInfo { name: "ko-en", repo: "onnx-community/opus-mt-ko-en" },
    OpusModelInfo { name: "en-zh", repo: "onnx-community/opus-mt-en-zh" },
    OpusModelInfo { name: "en-fr", repo: "onnx-community/opus-mt-en-fr" },
    OpusModelInfo { name: "en-de", repo: "onnx-community/opus-mt-en-de" },
    OpusModelInfo { name: "en-ru", repo: "onnx-community/opus-mt-en-ru" },
    OpusModelInfo { name: "en-ar", repo: "onnx-community/opus-mt-en-ar" },
];

const OPUS_ONNX_FILES: &[&str] = &[
    "encoder_model.onnx",      // ~200 MB
    "decoder_model.onnx",      // ~236 MB (non-merged, no KV cache)
];

const OPUS_ROOT_FILES: &[&str] = &[
    "tokenizer.json",
    "source.spm",
    "target.spm",
];

pub fn base_models_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("easytogokorea")
        .join("models")
}

pub fn is_opus_model_downloaded(name: &str) -> bool {
    let dir = base_models_dir().join(name);
    dir.join("encoder_model.onnx").exists()
        && dir.join("decoder_model.onnx").exists()
        && dir.join("tokenizer.json").exists()
}

pub async fn download_opus_models(progress: Option<&ProgressCallback>) -> Result<(), String> {
    let client = make_client()?;

    for model in OPUS_MODELS {
        let model_dir = base_models_dir().join(model.name);
        tokio::fs::create_dir_all(&model_dir).await
            .map_err(|e| format!("Failed to create dir: {e}"))?;

        for file_name in OPUS_ONNX_FILES {
            let file_path = model_dir.join(file_name);
            if file_path.exists() { continue; }
            let label = format!("{}/{}", model.name, file_name);
            let url = format!("https://huggingface.co/{}/resolve/main/onnx/{}", model.repo, file_name);
            download_file(&client, &url, &file_path, &label, progress).await
                .map_err(|e| format!("Failed to download {}: {e}", label))?;
        }

        for file_name in OPUS_ROOT_FILES {
            let file_path = model_dir.join(file_name);
            if file_path.exists() { continue; }
            let label = format!("{}/{}", model.name, file_name);
            let url = format!("https://huggingface.co/{}/resolve/main/{}", model.repo, file_name);
            download_file(&client, &url, &file_path, &label, progress).await
                .map_err(|e| format!("Failed to download {}: {e}", label))?;
        }
    }

    Ok(())
}

// ─── Common ───

fn make_client() -> Result<Client, String> {
    Client::builder()
        .timeout(std::time::Duration::from_secs(1800))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))
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

    let tmp_path = dest.with_extension("tmp");
    let mut file = tokio::fs::File::create(&tmp_path).await
        .map_err(|e| format!("File create error: {e}"))?;

    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Stream error: {e}"))?;
        file.write_all(&chunk).await
            .map_err(|e| format!("Write error: {e}"))?;
        downloaded += chunk.len() as u64;

        if let Some(cb) = &progress {
            cb(file_label, downloaded, total);
        }
    }

    file.flush().await.map_err(|e| format!("Flush error: {e}"))?;
    drop(file);

    tokio::fs::rename(&tmp_path, dest).await
        .map_err(|e| format!("Rename error: {e}"))?;

    Ok(())
}
