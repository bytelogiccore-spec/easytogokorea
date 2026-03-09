mod config;
mod error;
mod models;
mod clients;
mod routes;

use std::sync::Arc;

use axum::{Router, routing::get};
use reqwest::Client;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use config::AppConfig;

/// Shared application state passed to all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub http_client: Client,
}

#[tokio::main]
async fn main() {
    // Load .env file if present (silently ignore if absent)
    let _ = dotenvy::dotenv();

    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "easytogo_api_server=debug,tower_http=info".into()),
        )
        .init();

    // Load configuration
    let config = AppConfig::from_env();
    config.log_status();

    let bind_addr = format!("{}:{}", config.host, config.port);

    // Build shared state
    let state = AppState {
        config: Arc::new(config),
        http_client: Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client"),
    };

    // CORS — allow all origins for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(routes::health::check))
        // Tour API
        .route("/api/v1/tour/nearby", get(routes::tour::nearby))
        .route("/api/v1/tour/search", get(routes::tour::search))
        .route("/api/v1/tour/{content_id}", get(routes::tour::detail))
        .route("/api/v1/tour/festivals", get(routes::tour::festivals))
        // Weather
        .route("/api/v1/weather/forecast", get(routes::weather::forecast))
        .route("/api/v1/weather/current", get(routes::weather::current))
        // Air Quality
        .route("/api/v1/air-quality", get(routes::air_quality::realtime))
        // Exchange Rates
        .route("/api/v1/exchange/rates", get(routes::exchange::rates))
        // Transport
        .route("/api/v1/transport/bus/arrival", get(routes::transport::bus_arrival))
        .route("/api/v1/transport/subway/arrival", get(routes::transport::subway_arrival))
        // Emergency
        .route("/api/v1/emergency/rooms", get(routes::emergency::rooms))
        // Middleware
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    tracing::info!("🚀 EasyToGoKorea API Server starting on {bind_addr}");
    tracing::info!("📋 Endpoints:");
    tracing::info!("   GET /health");
    tracing::info!("   GET /api/v1/tour/nearby?lat=&lng=&radius=");
    tracing::info!("   GET /api/v1/tour/search?keyword=");
    tracing::info!("   GET /api/v1/tour/{{content_id}}");
    tracing::info!("   GET /api/v1/tour/festivals?start_date=");
    tracing::info!("   GET /api/v1/weather/forecast?nx=&ny=");
    tracing::info!("   GET /api/v1/weather/current?nx=&ny=");
    tracing::info!("   GET /api/v1/air-quality?station=");
    tracing::info!("   GET /api/v1/exchange/rates");
    tracing::info!("   GET /api/v1/transport/bus/arrival?station_id=");
    tracing::info!("   GET /api/v1/transport/subway/arrival?station=");
    tracing::info!("   GET /api/v1/emergency/rooms?region=");

    let listener = tokio::net::TcpListener::bind(&bind_addr).await
        .expect("Failed to bind address");

    axum::serve(listener, app).await
        .expect("Server error");
}
