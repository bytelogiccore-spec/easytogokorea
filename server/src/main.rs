mod config;
mod error;
mod models;
mod clients;
mod graphql;

use std::sync::Arc;

use axum::{Router, routing::get, response::{Html, IntoResponse}};
use async_graphql::{Schema, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use reqwest::Client;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use config::AppConfig;

use std::sync::atomic::{AtomicUsize, Ordering};

/// Shared application state passed to all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub http_client: Client,
    pub active_nodes: Arc<AtomicUsize>,
}

async fn graphql_handler(
    schema: axum::extract::Extension<graphql::AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn voyager() -> impl IntoResponse {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>EasyToGoKorea - GraphQL Voyager</title>
    <style>
        body { margin: 0; padding: 0; overflow: hidden; height: 100vh; }
        #voyager { height: 100vh; }
    </style>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-voyager@2.1.0/dist/voyager.css" />
    <script src="https://cdn.jsdelivr.net/npm/react@18/umd/react.production.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/react-dom@18/umd/react-dom.production.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/graphql-voyager@2.1.0/dist/voyager.standalone.js"></script>
</head>
<body>
    <div id="voyager"></div>
    <script>
        function introspectionProvider(query) {
            return fetch('/graphql', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ query }),
            }).then(response => response.json());
        }
        GraphQLVoyager.renderVoyager(document.getElementById('voyager'), {
            introspection: introspectionProvider,
            displayOptions: { sortByAlphabet: true },
        });
    </script>
</body>
</html>"#.to_string())
}

// Simple REST health check for load balancers/infrastructure
async fn health() -> impl IntoResponse {
    (axum::http::StatusCode::OK, "OK")
}

use axum::extract::ws::{WebSocketUpgrade, WebSocket};

async fn ws_heartbeat(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    state.active_nodes.fetch_add(1, Ordering::SeqCst);
    
    // Wait for the client to disconnect
    while let Some(Ok(_)) = socket.recv().await {
        // Just ignore incoming messages; we only care about connection status
    }
    
    state.active_nodes.fetch_sub(1, Ordering::SeqCst);
}

async fn active_nodes_count(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::Json<serde_json::Value> {
    let count = state.active_nodes.load(Ordering::SeqCst);
    axum::Json(serde_json::json!({ "count": count }))
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
        active_nodes: Arc::new(AtomicUsize::new(0)),
    };

    let schema = Schema::build(
        graphql::QueryRoot,
        graphql::MutationRoot,
        async_graphql::EmptySubscription,
    )
    .data(state.clone()) // Inject struct-based global context
    .finish();

    // CORS — allow all origins for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/health", get(health)) // REST Health Check
        .route("/", get(graphiql)) // GraphiQL IDE on base path
        .route("/voyager", get(voyager)) // GraphQL Voyager (schema visualization)
        .route("/graphql", axum::routing::post(graphql_handler).get(graphql_handler)) // GraphQL endpoint
        .route("/ws/heartbeat", axum::routing::get(ws_heartbeat))
        .route("/api/nodes/count", axum::routing::get(active_nodes_count))
        // Middleware
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(axum::extract::Extension(schema)) // Provides schema to handler
        .with_state(state.clone()); // Inject state for Axum Handlers

    tracing::info!("🚀 EasyToGoKorea GraphQL Server starting on {bind_addr}");
    tracing::info!("📋 Endpoints:");
    tracing::info!("   GET  /health   (REST Health Check)");
    tracing::info!("   GET  /         (GraphiQL IDE - 쿼리 테스트)");
    tracing::info!("   GET  /voyager  (GraphQL Voyager - 스키마 시각화)");
    tracing::info!("   POST /graphql  (GraphQL Endpoint)");

    let listener = tokio::net::TcpListener::bind(&bind_addr).await
        .expect("Failed to bind address");

    // Spawn the GraphQL server in a background task
    tokio::spawn(async move {
        axum::serve(listener, app).await
            .expect("Server error");
    });

    // Start Tauri app
    tauri::Builder::default()
        .setup(|app| {
            tracing::info!("Tauri application setup complete.");
            // Additional setup such as tray icons or deep linking can go here
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
