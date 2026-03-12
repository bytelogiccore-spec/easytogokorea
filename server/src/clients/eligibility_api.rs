use std::sync::Arc;
use reqwest::Client;
use serde_json::json;

use crate::config::AppConfig;
use crate::error::AppError;
use crate::models::eligibility::{
    EligibilityCheckRequest, EligibilityCheckResponse, EligibilityStatus,
};

// Note: This implements a generic structure that simulates a call to an Eligibility API
// like pVerify or Stedi. In a real environment, you would use their specific endpoint URL
// and map their specific JSON payloads.
const ELIGIBILITY_API_URL: &str = "https://sandbox.pverify.com/api/EligibilityInfo";

pub struct EligibilityClient {
    config: Arc<AppConfig>,
    http_client: Client,
}

impl EligibilityClient {
    pub fn new(config: Arc<AppConfig>, http_client: Client) -> Self {
        Self {
            config,
            http_client,
        }
    }

    /// Check patient healthcare eligibility via third-party API.
    pub async fn check_eligibility(
        &self,
        req: &EligibilityCheckRequest,
    ) -> Result<EligibilityCheckResponse, AppError> {
        let api_key = match &self.config.eligibility_api_key {
            Some(key) => key,
            None => {
                // Return a mocked successful response if no key is configured
                tracing::warn!("ELIGIBILITY_API_KEY is not configured. Returning mock data.");
                return Ok(self.mock_response(req));
            }
        };

        // Construct the expected payload for the external service
        // Examples based loosely on common standardized APIs (X12 270/271 mappings)
        let payload = json!({
            "patient": {
                "id": req.patient_id,
                "firstName": req.patient_first_name,
                "lastName": req.patient_last_name,
                "dob": req.patient_dob
            },
            "provider": {
                "npi": req.provider_npi
            },
            "serviceTypeCode": req.service_type.clone().unwrap_or_else(|| "30".to_string())
        });

        // Make the HTTP request
        let response = self
            .http_client
            .post(ELIGIBILITY_API_URL)
            // .header("Authorization", format!("Bearer {}", api_key)) // Assuming Bearer token auth
            .header("client-secret", api_key) // pVerify style secret header
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Eligibility API request failed: {}", e);
                AppError::ExternalApi(format!("Eligibility check request failed: {}", e))
            })?;

        if !response.status().is_success() {
            tracing::error!("Eligibility API returned error status: {}", response.status());
            return Err(AppError::ExternalApi(format!(
                "Eligibility API returned error status: {}",
                response.status()
            )));
        }

        let body: serde_json::Value = response.json().await.map_err(|e: reqwest::Error| {
            tracing::error!("Failed to parse Eligibility API response: {}", e);
            AppError::ExternalApi("Invalid JSON response from Eligibility API".to_string())
        })?;

        // In a real integration, you would strongly map `body` to Stedi/pVerify response types.
        // For demonstration, we simply parse some fields and construct our normalized response.
        let status = match body.get("status").and_then(|s: &serde_json::Value| s.as_str()) {
            Some("Active") => EligibilityStatus::Active,
            Some("Inactive") => EligibilityStatus::Inactive,
            Some("Pending") => EligibilityStatus::Pending,
            _ => EligibilityStatus::Unknown,
        };

        let subscriber_id = body
            .get("subscriberId")
            .and_then(|s: &serde_json::Value| s.as_str())
            .unwrap_or("UNKNOWN")
            .to_string();

        let plan_name = body
            .get("planName")
            .and_then(|s: &serde_json::Value| s.as_str())
            .map(|s: &str| s.to_string());

        let active_coverage = body
            .get("activeCoverage")
            .and_then(|b: &serde_json::Value| b.as_bool())
            .unwrap_or(false);

        let copay = body
            .get("copayAmount")
            .and_then(|n: &serde_json::Value| n.as_f64());

        Ok(EligibilityCheckResponse {
            status,
            subscriber_id,
            plan_name,
            active_coverage,
            copay,
            raw_response: Some(body),
        })
    }

    fn mock_response(&self, req: &EligibilityCheckRequest) -> EligibilityCheckResponse {
        EligibilityCheckResponse {
            status: EligibilityStatus::Active,
            subscriber_id: format!("SUB-{}", req.patient_id),
            plan_name: Some("EasyToGo Premium Plan".to_string()),
            active_coverage: true,
            copay: Some(15.00),
            raw_response: None,
        }
    }
}
