pub mod tour;
pub mod weather;
pub mod air_quality;
pub mod exchange;
pub mod transport;
pub mod emergency;
pub mod eligibility;

use serde::Serialize;

/// Standard API response wrapper.
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached: Option<bool>,
}

impl<T: Serialize> ApiResponse<T> {
    #[allow(dead_code)]
    pub fn ok(data: T) -> Self {
        Self { success: true, data, cached: None }
    }

    #[allow(dead_code)]
    pub fn cached(data: T) -> Self {
        Self { success: true, data, cached: Some(true) }
    }
}
