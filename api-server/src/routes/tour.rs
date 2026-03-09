use axum::{extract::{Path, Query, State}, Json};
use serde::Deserialize;

use crate::error::AppError;
use crate::models::ApiResponse;
use crate::models::tour::{Attraction, AttractionList};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct NearbyParams {
    pub lat: f64,
    pub lng: f64,
    pub radius: Option<i32>,
    pub content_type: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub keyword: String,
    pub content_type: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct FestivalParams {
    pub start_date: String,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

pub async fn nearby(
    State(state): State<AppState>,
    Query(params): Query<NearbyParams>,
) -> Result<Json<ApiResponse<AttractionList>>, AppError> {
    let key = state.config.tour_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("TourAPI".into()))?;

    let client = crate::clients::tour_api::TourApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    let result = client.get_nearby(
        params.lat,
        params.lng,
        params.radius.unwrap_or(5000),
        params.content_type.as_deref(),
        params.page.unwrap_or(1),
        params.size.unwrap_or(20),
    ).await?;

    Ok(Json(ApiResponse::ok(result)))
}

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<ApiResponse<AttractionList>>, AppError> {
    let key = state.config.tour_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("TourAPI".into()))?;

    let client = crate::clients::tour_api::TourApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    let result = client.search(
        &params.keyword,
        params.content_type.as_deref(),
        params.page.unwrap_or(1),
        params.size.unwrap_or(20),
    ).await?;

    Ok(Json(ApiResponse::ok(result)))
}

pub async fn detail(
    State(state): State<AppState>,
    Path(content_id): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Attraction>>, AppError> {
    let key = state.config.tour_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("TourAPI".into()))?;

    let client = crate::clients::tour_api::TourApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    let content_type = params.get("content_type").map(|s| s.as_str());
    let result = client.get_detail(&content_id, content_type).await?;

    Ok(Json(ApiResponse::ok(result)))
}

pub async fn festivals(
    State(state): State<AppState>,
    Query(params): Query<FestivalParams>,
) -> Result<Json<ApiResponse<AttractionList>>, AppError> {
    let key = state.config.tour_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("TourAPI".into()))?;

    let client = crate::clients::tour_api::TourApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    let result = client.get_festivals(
        &params.start_date,
        params.page.unwrap_or(1),
        params.size.unwrap_or(20),
    ).await?;

    Ok(Json(ApiResponse::ok(result)))
}
