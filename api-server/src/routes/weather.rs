use axum::{extract::{Query, State}, Json};
use chrono::Local;
use serde::Deserialize;

use crate::error::AppError;
use crate::models::ApiResponse;
use crate::models::weather::WeatherForecast;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct WeatherParams {
    pub nx: i32,
    pub ny: i32,
    pub base_date: Option<String>,
    pub base_time: Option<String>,
}

pub async fn forecast(
    State(state): State<AppState>,
    Query(params): Query<WeatherParams>,
) -> Result<Json<ApiResponse<Vec<WeatherForecast>>>, AppError> {
    let key = state.config.weather_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("Weather".into()))?;

    let client = crate::clients::weather_api::WeatherApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    // Default base_date/time: today, 0500 (KMA publishes at 0200, 0500, 0800, 1100, 1400, 1700, 2000, 2300)
    let base_date = params.base_date.unwrap_or_else(|| Local::now().format("%Y%m%d").to_string());
    let base_time = params.base_time.unwrap_or_else(|| "0500".to_string());

    let result = client.get_forecast(params.nx, params.ny, &base_date, &base_time).await?;

    Ok(Json(ApiResponse::ok(result)))
}

pub async fn current(
    State(state): State<AppState>,
    Query(params): Query<WeatherParams>,
) -> Result<Json<ApiResponse<Vec<WeatherForecast>>>, AppError> {
    let key = state.config.weather_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("Weather".into()))?;

    let client = crate::clients::weather_api::WeatherApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    let now = Local::now();
    let base_date = params.base_date.unwrap_or_else(|| now.format("%Y%m%d").to_string());
    // Ultra-short observation: use the latest full hour
    let base_time = params.base_time.unwrap_or_else(|| {
        let hour = now.format("%H").to_string();
        format!("{hour}00")
    });

    let result = client.get_current(params.nx, params.ny, &base_date, &base_time).await?;

    Ok(Json(ApiResponse::ok(result)))
}
