mod ble;
mod server;

use ble::{BleChat, ChatMessage, ChatStatus};
use std::sync::{Arc, Mutex};
use tauri::State;

struct AppState {
    chat: Mutex<BleChat>,
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
        })
        .invoke_handler(tauri::generate_handler![
            start_chat_host,
            send_chat_message,
            get_chat_messages,
            get_chat_status,
            disconnect_chat,
            generate_qr,
            ping_api_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
