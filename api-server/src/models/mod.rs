pub mod tour;
pub mod weather;
pub mod air_quality;
pub mod exchange;
pub mod transport;
pub mod emergency;

use serde::Serialize;

/// Standard API response wrapper.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached: Option<bool>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { success: true, data, cached: None }
    }

    pub fn cached(data: T) -> Self {
        Self { success: true, data, cached: Some(true) }
    }
}
