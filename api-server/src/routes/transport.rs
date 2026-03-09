use axum::{extract::{Query, State}, Json};
use serde::Deserialize;

use crate::error::AppError;
use crate::models::ApiResponse;
use crate::models::transport::{BusArrival, SubwayArrival};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct BusParams {
    pub station_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SubwayParams {
    pub station: String,
}

pub async fn bus_arrival(
    State(state): State<AppState>,
    Query(params): Query<BusParams>,
) -> Result<Json<ApiResponse<BusArrival>>, AppError> {
    let key = state.config.seoul_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("Seoul Transport".into()))?;

    let client = crate::clients::transport_api::TransportApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    let result = client.get_bus_arrival(&params.station_id).await?;

    Ok(Json(ApiResponse::ok(result)))
}

pub async fn subway_arrival(
    State(state): State<AppState>,
    Query(params): Query<SubwayParams>,
) -> Result<Json<ApiResponse<SubwayArrival>>, AppError> {
    let key = state.config.seoul_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("Seoul Transport".into()))?;

    let client = crate::clients::transport_api::TransportApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    let result = client.get_subway_arrival(&params.station).await?;

    Ok(Json(ApiResponse::ok(result)))
}
