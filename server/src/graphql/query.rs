use async_graphql::{Context, Object, Result};
use crate::AppState;

// Internal dependencies
use crate::models::{
    tour::{Attraction, AttractionList},
    weather::WeatherForecast,
    air_quality::AirQuality,
    exchange::ExchangeRateList,
    transport::{BusArrival, SubwayArrival},
    emergency::EmergencyRoom,
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Return a basic health check greeting
    async fn health_check(&self) -> &str {
        "Easytogo Korea GraphQL API is running smoothly!"
    }

    /// Tour API: Get nearby attractions based on map coordinates
    async fn tour_nearby(
        &self,
        ctx: &Context<'_>,
        lat: f64,
        lng: f64,
        radius: Option<i32>,
        content_type_id: Option<String>,
        page: Option<i32>,
        num_of_rows: Option<i32>,
    ) -> Result<AttractionList> {
        let state = ctx.data::<AppState>().unwrap();
        // Here we map to our existing TourApiClient
        let client = crate::clients::tour_api::TourApiClient::new(
            state.http_client.clone(),
            state.config.tour_api_key.clone().unwrap_or_default(),
        );
        let radius = radius.unwrap_or(1000);
        let page = page.unwrap_or(1);
        let num_of_rows = num_of_rows.unwrap_or(10);
        let result = client.get_nearby(lat, lng, radius, content_type_id.as_deref(), page, num_of_rows).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Tour API: Search attractions by keyword
    async fn tour_search(
        &self,
        ctx: &Context<'_>,
        keyword: String,
        content_type_id: Option<String>,
        page: Option<i32>,
        num_of_rows: Option<i32>,
    ) -> Result<AttractionList> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::tour_api::TourApiClient::new(
            state.http_client.clone(),
            state.config.tour_api_key.clone().unwrap_or_default(),
        );
        let page = page.unwrap_or(1);
        let num_of_rows = num_of_rows.unwrap_or(10);
        let result = client.search(&keyword, content_type_id.as_deref(), page, num_of_rows).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Tour API: Detail information by content ID
    async fn tour_detail(
        &self,
        ctx: &Context<'_>,
        content_id: String,
        content_type_id: Option<String>,
    ) -> Result<Attraction> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::tour_api::TourApiClient::new(
            state.http_client.clone(),
            state.config.tour_api_key.clone().unwrap_or_default(),
        );
        let result = client.get_detail(&content_id, content_type_id.as_deref()).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Tour API: Festival list starting from a date (YYYYMMDD)
    async fn tour_festivals(
        &self,
        ctx: &Context<'_>,
        start_date: String,
        page: Option<i32>,
        num_of_rows: Option<i32>,
    ) -> Result<AttractionList> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::tour_api::TourApiClient::new(
            state.http_client.clone(),
            state.config.tour_api_key.clone().unwrap_or_default(),
        );
        let page = page.unwrap_or(1);
        let num_of_rows = num_of_rows.unwrap_or(10);
        let result = client.get_festivals(&start_date, page, num_of_rows).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Weather API: Short-term forecast for an X, Y grid point
    async fn weather_forecast(
        &self,
        ctx: &Context<'_>,
        nx: i32,
        ny: i32,
        base_date: String,
        base_time: String,
    ) -> Result<Vec<WeatherForecast>> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::weather_api::WeatherApiClient::new(
            state.http_client.clone(),
            state.config.weather_api_key.clone().unwrap_or_default(),
        );
        let result = client.get_forecast(nx, ny, &base_date, &base_time).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Weather API: Current mid-term weather overview
    async fn weather_current(
        &self,
        ctx: &Context<'_>,
        nx: i32,
        ny: i32,
        base_date: String,
        base_time: String,
    ) -> Result<Vec<WeatherForecast>> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::weather_api::WeatherApiClient::new(
            state.http_client.clone(),
            state.config.weather_api_key.clone().unwrap_or_default(),
        );
        let result = client.get_current(nx, ny, &base_date, &base_time).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Air Quality API: Latest measurements from a specific station
    async fn air_quality(
        &self,
        ctx: &Context<'_>,
        station_name: String,
    ) -> Result<AirQuality> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::air_quality_api::AirQualityApiClient::new(
            state.http_client.clone(),
            state.config.airkorea_api_key.clone().unwrap_or_default(),
        );
        let result = client.get_realtime(&station_name).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Exchange Rates API: Get BOK current exchange rates vs KRW
    async fn exchange_rates(&self, ctx: &Context<'_>, date: String) -> Result<ExchangeRateList> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::exchange_api::ExchangeApiClient::new(
            state.http_client.clone(),
            state.config.bok_api_key.clone().unwrap_or_default(),
        );
        let result = client.get_rates(&date).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Transport API: Seoul Bus real-time arrival info mapping
    async fn bus_arrival(
        &self,
        ctx: &Context<'_>,
        station_id: String,
    ) -> Result<BusArrival> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::transport_api::TransportApiClient::new(
            state.http_client.clone(),
            state.config.seoul_api_key.clone().unwrap_or_default(),
        );
        let result = client.get_bus_arrival(&station_id).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Transport API: Seoul Subway real-time arrival info mapping
    async fn subway_arrival(
        &self,
        ctx: &Context<'_>,
        station_name: String,
    ) -> Result<SubwayArrival> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::transport_api::TransportApiClient::new(
            state.http_client.clone(),
            state.config.seoul_api_key.clone().unwrap_or_default(),
        );
        let result = client.get_subway_arrival(&station_name).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Emergency API: Find hospitals/ERs in a specific region
    async fn emergency_rooms(
        &self,
        ctx: &Context<'_>,
        q0: String, // City/Do (e.g., "서울특별시")
        q1: Option<String>, // Gu/Gun (e.g., "강남구")
    ) -> Result<Vec<EmergencyRoom>> {
        let state = ctx.data::<AppState>().unwrap();
        let client = crate::clients::emergency_api::EmergencyApiClient::new(
            state.http_client.clone(),
            state.config.emergency_api_key.clone().unwrap_or_default(),
        );
        let result = client.get_emergency_rooms(&q0, q1.as_deref()).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }
}
