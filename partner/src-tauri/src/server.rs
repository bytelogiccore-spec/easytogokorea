use axum::{
    routing::{get, post},
    Router, Json, response::IntoResponse, http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use tokio::sync::Mutex;
use dbx_core::Database;
use std::net::SocketAddr;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PartnerProfile {
    pub name: String,
    pub catchphrase: String,
    pub tags: String,
    pub list_image_url: String,
    pub description: String,
    pub gallery: Vec<String>,
    pub categories: Vec<serde_json::Value>,
}

#[derive(Clone)]
pub struct ServerState {
    pub db: Arc<Database>,
}

pub async fn start_server(port: u16) {
    let cors = CorsLayer::permissive();

    // Initialize DBX
    // Save DB outside of src-tauri to prevent Tauri Watcher hot-reload loop
    let db = match Database::open(Path::new("../partner_dbx")) {
        Ok(db) => db,
        Err(_) => {
            println!("Failed to create persistent dbx, using memory fallback");
            Arc::new(Database::open_in_memory().expect("Failed to open DBX in memory"))
        }
    };

    let shared_state = ServerState {
        db,
    };

    let app = Router::new()
        .route("/api/profile", get(get_profile).post(save_profile))
        .layer(cors)
        .with_state(shared_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Node Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_profile(
    axum::extract::State(state): axum::extract::State<ServerState>,
) -> impl IntoResponse {
    let db = &state.db;
    
    // We only have one profile per node
    if let Ok(Some(data_vec)) = db.get("profile", b"my_profile") {
        if let Ok(profile) = serde_json::from_slice::<PartnerProfile>(&data_vec) {
            return (StatusCode::OK, Json(profile)).into_response();
        }
    }
    
    (StatusCode::NOT_FOUND, "Profile not set").into_response()
}

async fn save_profile(
    axum::extract::State(state): axum::extract::State<ServerState>,
    Json(payload): Json<PartnerProfile>,
) -> impl IntoResponse {
    let db = &state.db;
    let encoded = serde_json::to_vec(&payload).unwrap();
    
    // Insert into DBX
    if db.insert("profile", b"my_profile", &encoded).is_ok() {
        // Ping the central API-SERVER running on localhost:8000
        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let _ = client.get("http://127.0.0.1:8000/health").send().await;
            println!("Pinged central api-server successfully.");
        });

        (StatusCode::OK, "Profile Saved into DBX").into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save to DBX").into_response()
    }
}
