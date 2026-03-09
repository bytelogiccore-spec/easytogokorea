use axum::{extract::{Query, State}, Json};
use serde::Deserialize;

use crate::error::AppError;
use crate::models::ApiResponse;
use crate::models::emergency::EmergencyRoom;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct EmergencyParams {
    pub region: String,
    pub sub_region: Option<String>,
}

pub async fn rooms(
    State(state): State<AppState>,
    Query(params): Query<EmergencyParams>,
) -> Result<Json<ApiResponse<Vec<EmergencyRoom>>>, AppError> {
    let key = state.config.emergency_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("Emergency".into()))?;

    let client = crate::clients::emergency_api::EmergencyApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    let result = client.get_emergency_rooms(
        &params.region,
        params.sub_region.as_deref(),
    ).await?;

    Ok(Json(ApiResponse::ok(result)))
}
