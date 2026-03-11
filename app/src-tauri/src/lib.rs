// AR bridge commands — called from Svelte via Tauri invoke()
// On Android, these delegate to the Kotlin ARCore plugin via JNI.
// On Desktop/iOS, they return fallback responses.

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Check if AR is available on this device
#[tauri::command]
fn ar_check_availability() -> serde_json::Value {
    // On desktop, AR is not available
    // On Android, this will be overridden by the Kotlin plugin
    serde_json::json!({
        "available": false,
        "platform": std::env::consts::OS,
        "reason": "AR requires a mobile device with ARCore/ARKit support"
    })
}

/// Start an AR session
#[tauri::command]
fn ar_start_session() -> serde_json::Value {
    serde_json::json!({
        "success": false,
        "reason": "AR sessions require ARCore (Android) or ARKit (iOS)"
    })
}

/// Perform a hit test at screen coordinates
#[tauri::command]
fn ar_hit_test(x: f32, y: f32) -> serde_json::Value {
    serde_json::json!({
        "hit": false,
        "x": x,
        "y": y,
        "reason": "Hit test requires an active AR session on a mobile device"
    })
}

/// Get current AR tracking state
#[tauri::command]
fn ar_get_tracking_info() -> serde_json::Value {
    serde_json::json!({
        "available": false,
        "tracking": false,
        "planes": 0
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            ar_check_availability,
            ar_start_session,
            ar_hit_test,
            ar_get_tracking_info
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title("EasyToGo Korea");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
