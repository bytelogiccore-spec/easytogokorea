use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // Enable camera/media permissions on the WebView
            if let Some(window) = app.get_webview_window("main") {
                // On desktop, we need to handle permission requests
                // The WebView will prompt for camera access via getUserMedia
                let _ = window.set_title("EasyToGo Korea");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
