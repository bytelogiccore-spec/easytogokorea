use axum::{extract::{Query, State}, Json};
use serde::Deserialize;

use crate::error::AppError;
use crate::models::ApiResponse;
use crate::models::air_quality::AirQuality;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct AirQualityParams {
    pub station: String,
}

pub async fn realtime(
    State(state): State<AppState>,
    Query(params): Query<AirQualityParams>,
) -> Result<Json<ApiResponse<AirQuality>>, AppError> {
    let key = state.config.airkorea_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("AirKorea".into()))?;

    let client = crate::clients::air_quality_api::AirQualityApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    let result = client.get_realtime(&params.station).await?;

    Ok(Json(ApiResponse::ok(result)))
}
