// AR is now handled natively on Android via ArBridge (@JavascriptInterface).
// These Tauri commands serve as desktop/iOS fallbacks.

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

/// Save e-arrival card data locally (on-device only, no cloud)
#[tauri::command]
fn save_arrival_data(data: String) -> Result<String, String> {
    let dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("easytogokorea");
    std::fs::create_dir_all(&dir).map_err(|e| format!("Dir error: {e}"))?;
    let path = dir.join("arrival_data.json");
    std::fs::write(&path, &data).map_err(|e| format!("Write error: {e}"))?;
    Ok(format!("Saved to {}", path.display()))
}

/// Load previously saved e-arrival card data
#[tauri::command]
fn load_arrival_data() -> Result<String, String> {
    let path = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("easytogokorea")
        .join("arrival_data.json");
    if path.exists() {
        std::fs::read_to_string(&path).map_err(|e| format!("Read error: {e}"))
    } else {
        Ok("{}".to_string())
    }
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
            ar_get_tracking_info,
            save_arrival_data,
            load_arrival_data
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
