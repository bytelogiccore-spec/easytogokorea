mod ble;
mod server;

use ble::{BleChat, ChatMessage, ChatStatus};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tauri::{State, AppHandle, Emitter};

struct AppState {
    chat: Mutex<BleChat>,
    translator: Arc<translator::Translator>,
}

#[tauri::command]
fn start_chat_host(pin: String, state: State<AppState>) -> ChatStatus {
    let mut chat = state.chat.lock().unwrap();
    chat.start_host(pin);
    chat.status()
}

#[tauri::command]
fn send_chat_message(text: String, state: State<AppState>) -> bool {
    let mut chat = state.chat.lock().unwrap();
    chat.add_message(ChatMessage {
        msg_type: "sent".into(),
        text,
    });
    true
}

#[tauri::command]
fn get_chat_messages(state: State<AppState>) -> Vec<ChatMessage> {
    let mut chat = state.chat.lock().unwrap();
    chat.drain_messages()
}

#[tauri::command]
fn get_chat_status(state: State<AppState>) -> ChatStatus {
    let chat = state.chat.lock().unwrap();
    chat.status()
}

#[tauri::command]
fn disconnect_chat(state: State<AppState>) {
    let mut chat = state.chat.lock().unwrap();
    *chat = BleChat::new();
}

#[tauri::command]
fn generate_qr(data: String) -> String {
    ble::generate_qr_base64(&data)
}

#[tauri::command]
async fn ping_api_server() -> Result<String, String> {
    let client = reqwest::Client::new();
    match client.get("http://127.0.0.1:8000/health").send().await {
        Ok(res) => {
            if res.status().is_success() {
                Ok("Connected to local api-server!".to_string())
            } else {
                Err(format!("api-server responded with: {}", res.status()))
            }
        }
        Err(e) => Err(format!("Failed to connect to api-server: {}", e)),
    }
}

// ─── Translation Commands ───

#[tauri::command]
fn translate_text(
    text: String,
    source: String,
    target: String,
    state: State<AppState>,
) -> Result<String, String> {
    state.translator.translate(&text, &source, &target)
}

#[tauri::command]
fn translate_all(
    text: String,
    source: String,
    state: State<AppState>,
) -> HashMap<String, String> {
    state.translator.translate_all(&text, &source)
}

#[tauri::command]
fn check_models_status() -> HashMap<String, bool> {
    let mut status = HashMap::new();
    for model in translator::MODELS {
        status.insert(model.name.to_string(), translator::is_model_downloaded(model.name));
    }
    status
}

#[tauri::command]
async fn download_translation_models(app: AppHandle) -> Result<String, String> {
    let progress_cb: translator::ProgressCallback = Box::new(move |model_name, downloaded, total| {
        let _ = app.emit("translation-download-progress", serde_json::json!({
            "model": model_name,
            "downloaded": downloaded,
            "total": total,
        }));
    });

    translator::download_all_models(Some(&progress_cb)).await?;
    Ok("All models downloaded".to_string())
}

#[tauri::command]
fn get_supported_languages() -> Vec<(String, String)> {
    translator::SUPPORTED_LANGUAGES
        .iter()
        .map(|&(code, label)| (code.to_string(), label.to_string()))
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Spawn Axum server on a background thread using DBX
    tauri::async_runtime::spawn(async {
        server::start_server(3333).await;
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            chat: Mutex::new(BleChat::new()),
            translator: Arc::new(translator::Translator::new()),
        })
        .invoke_handler(tauri::generate_handler![
            start_chat_host,
            send_chat_message,
            get_chat_messages,
            get_chat_status,
            disconnect_chat,
            generate_qr,
            ping_api_server,
            translate_text,
            translate_all,
            check_models_status,
            download_translation_models,
            get_supported_languages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

