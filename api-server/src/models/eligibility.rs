use serde::{Deserialize, Serialize};
use async_graphql::{SimpleObject, InputObject, Enum};

#[derive(Debug, Deserialize, Serialize, Clone, InputObject)]
pub struct EligibilityCheckRequest {
    pub patient_id: String,
    pub patient_first_name: String,
    pub patient_last_name: String,
    pub patient_dob: String, // YYYY-MM-DD
    pub provider_npi: String,
    pub service_type: Option<String>, // Medical service type code, e.g., "30" for Health Benefit Plan Coverage
}

#[derive(Debug, Deserialize, Serialize, Clone, SimpleObject)]
pub struct EligibilityCheckResponse {
    pub status: EligibilityStatus,
    pub subscriber_id: String,
    pub plan_name: Option<String>,
    pub active_coverage: bool,
    pub copay: Option<f64>,
    pub raw_response: Option<serde_json::Value>, // Store the raw third-party response for debugging
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Enum, Copy, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EligibilityStatus {
    Active,
    Inactive,
    Pending,
    Unknown,
    Error,
}
