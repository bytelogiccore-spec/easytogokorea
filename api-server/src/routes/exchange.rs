use axum::{extract::State, Json};
use chrono::Local;

use crate::error::AppError;
use crate::models::ApiResponse;
use crate::models::exchange::ExchangeRateList;
use crate::AppState;

pub async fn rates(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<ExchangeRateList>>, AppError> {
    let key = state.config.bok_api_key.as_ref()
        .ok_or_else(|| AppError::ApiKeyMissing("BOK".into()))?;

    let client = crate::clients::exchange_api::ExchangeApiClient::new(
        state.http_client.clone(),
        key.clone(),
    );

    // Use today's date; BOK API may not have data for weekends/holidays
    let date = Local::now().format("%Y%m%d").to_string();
    let result = client.get_rates(&date).await?;

    Ok(Json(ApiResponse::ok(result)))
}
