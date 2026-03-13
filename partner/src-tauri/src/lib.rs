mod ble;
mod server;

use ble::{BleChat, ChatMessage, ChatStatus, NearbyDevice};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use tauri::{State, AppHandle, Emitter};

struct AppState {
    chat: Mutex<BleChat>,
    translator: Arc<translator::Translator>,
}

// ─── BLE Chat Commands ───

#[tauri::command]
async fn init_ble(state: State<'_, AppState>) -> Result<String, String> {
    let mut chat = state.chat.lock().await;
    chat.init_adapter().await?;
    Ok("BLE adapter initialized".to_string())
}

#[tauri::command]
async fn start_ble_scan(state: State<'_, AppState>) -> Result<String, String> {
    let mut chat = state.chat.lock().await;
    chat.start_scan().await?;
    Ok("Scanning started".to_string())
}

#[tauri::command]
async fn stop_ble_scan(state: State<'_, AppState>) -> Result<String, String> {
    let chat = state.chat.lock().await;
    chat.stop_scan().await?;
    Ok("Scanning stopped".to_string())
}

#[tauri::command]
async fn get_nearby_devices(state: State<'_, AppState>) -> Result<Vec<NearbyDevice>, String> {
    let mut chat = state.chat.lock().await;
    chat.get_nearby_devices().await
}

#[tauri::command]
async fn connect_ble_device(device_id: String, state: State<'_, AppState>) -> Result<String, String> {
    let mut chat = state.chat.lock().await;
    chat.connect_to_device(&device_id).await?;
    Ok("Connected".to_string())
}

#[tauri::command]
async fn send_ble_message(text: String, state: State<'_, AppState>) -> Result<bool, String> {
    let mut chat = state.chat.lock().await;
    chat.send_ble_message(&text).await?;
    Ok(true)
}

#[tauri::command]
async fn start_chat_host(pin: String, state: State<'_, AppState>) -> Result<ChatStatus, String> {
    let mut chat = state.chat.lock().await;
    chat.start_host(pin);
    Ok(chat.status())
}

#[tauri::command]
async fn send_chat_message(text: String, state: State<'_, AppState>) -> Result<bool, String> {
    let mut chat = state.chat.lock().await;
    chat.add_message(ChatMessage {
        msg_type: "sent".into(),
        text,
    });
    Ok(true)
}

#[tauri::command]
async fn get_chat_messages(state: State<'_, AppState>) -> Result<Vec<ChatMessage>, String> {
    let mut chat = state.chat.lock().await;
    Ok(chat.drain_messages())
}

#[tauri::command]
async fn get_chat_status(state: State<'_, AppState>) -> Result<ChatStatus, String> {
    let chat = state.chat.lock().await;
    Ok(chat.status())
}

#[tauri::command]
async fn disconnect_chat(state: State<'_, AppState>) -> Result<(), String> {
    let mut chat = state.chat.lock().await;
    *chat = BleChat::new();
    Ok(())
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
fn check_models_status(state: State<AppState>) -> HashMap<String, bool> {
    state.translator.available_engines()
}

#[tauri::command]
async fn download_nllb_model(app: AppHandle) -> Result<String, String> {
    let progress_cb: translator::ProgressCallback = Box::new(move |file_name, downloaded, total| {
        let _ = app.emit("translation-download-progress", serde_json::json!({
            "file": file_name,
            "downloaded": downloaded,
            "total": total,
        }));
    });
    translator::download_nllb(Some(&progress_cb)).await?;
    Ok("NLLB-200 model downloaded successfully".to_string())
}

#[tauri::command]
async fn download_opus_models(app: AppHandle) -> Result<String, String> {
    let progress_cb: translator::ProgressCallback = Box::new(move |file_name, downloaded, total| {
        let _ = app.emit("translation-download-progress", serde_json::json!({
            "file": file_name,
            "downloaded": downloaded,
            "total": total,
        }));
    });
    translator::download_opus_models(Some(&progress_cb)).await?;
    Ok("Opus-MT models downloaded successfully".to_string())
}

#[tauri::command]
fn get_supported_languages() -> Vec<(String, String)> {
    translator::SUPPORTED_LANGUAGES
        .iter()
        .map(|&(code, label)| (code.to_string(), label.to_string()))
        .collect()
}

#[tauri::command]
fn get_engine(state: State<AppState>) -> String {
    format!("{:?}", state.translator.current_engine())
}

#[tauri::command]
fn set_engine(engine: String, state: State<AppState>) -> Result<String, String> {
    let engine_type = match engine.as_str() {
        "OpusMT" | "opus-mt" | "opus" => translator::EngineType::OpusMT,
        "Nllb200" | "nllb-200" | "nllb" => translator::EngineType::Nllb200,
        _ => return Err(format!("Unknown engine: {engine}")),
    };
    state.translator.set_engine(engine_type);
    Ok(format!("Engine set to {engine}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Spawn Axum server on a background thread using DBX
    tauri::async_runtime::spawn(async {
        server::start_server(3333).await;
    });

    let translator = Arc::new(translator::Translator::new());

    // Preload default engine in background thread
    let preload_translator = translator.clone();
    std::thread::Builder::new()
        .name("translator-preload".to_string())
        .stack_size(8 * 1024 * 1024) // 8MB stack for ONNX
        .spawn(move || {
            preload_translator.preload();
        })
        .ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            chat: Mutex::new(BleChat::new()),
            translator,
        })
        .invoke_handler(tauri::generate_handler![
            // BLE commands
            init_ble,
            start_ble_scan,
            stop_ble_scan,
            get_nearby_devices,
            connect_ble_device,
            send_ble_message,
            // Legacy chat
            start_chat_host,
            send_chat_message,
            get_chat_messages,
            get_chat_status,
            disconnect_chat,
            generate_qr,
            ping_api_server,
            // Translation
            translate_text,
            translate_all,
            check_models_status,
            download_nllb_model,
            download_opus_models,
            get_supported_languages,
            get_engine,
            set_engine,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
