use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// Unified error type for the application.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("External API error: {0}")]
    ExternalApi(String),

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("API key not configured: {0}")]
    #[allow(dead_code)]
    ApiKeyMissing(String),

    #[error("Invalid parameter: {0}")]
    #[allow(dead_code)]
    InvalidParam(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::ExternalApi(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            AppError::HttpClient(e) => (StatusCode::BAD_GATEWAY, e.to_string()),
            AppError::JsonParse(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::ApiKeyMissing(api) => (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("{api} API key is not configured. Set it in .env file."),
            ),
            AppError::InvalidParam(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
        };

        let body = json!({
            "error": message,
            "status": status.as_u16(),
        });

        (status, axum::Json(body)).into_response()
    }
}
