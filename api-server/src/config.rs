use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub tour_api_key: Option<String>,
    pub weather_api_key: Option<String>,
    pub airkorea_api_key: Option<String>,
    pub bok_api_key: Option<String>,
    pub seoul_api_key: Option<String>,
    pub emergency_api_key: Option<String>,
}

impl AppConfig {
    /// Load configuration from environment variables.
    /// API keys are optional — the server can start without them.
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            tour_api_key: env::var("KTO_TOUR_API_KEY").ok().filter(|k| !k.is_empty() && !k.starts_with("YOUR_")),
            weather_api_key: env::var("KMA_WEATHER_API_KEY").ok().filter(|k| !k.is_empty() && !k.starts_with("YOUR_")),
            airkorea_api_key: env::var("AIRKOREA_API_KEY").ok().filter(|k| !k.is_empty() && !k.starts_with("YOUR_")),
            bok_api_key: env::var("BOK_API_KEY").ok().filter(|k| !k.is_empty() && !k.starts_with("YOUR_")),
            seoul_api_key: env::var("SEOUL_API_KEY").ok().filter(|k| !k.is_empty() && !k.starts_with("YOUR_")),
            emergency_api_key: env::var("EMERGENCY_API_KEY").ok().filter(|k| !k.is_empty() && !k.starts_with("YOUR_")),
        }
    }

    /// Check which API keys are configured.
    pub fn log_status(&self) {
        let apis = [
            ("TourAPI (관광)", &self.tour_api_key),
            ("Weather (날씨)", &self.weather_api_key),
            ("AirKorea (대기질)", &self.airkorea_api_key),
            ("BOK (환율)", &self.bok_api_key),
            ("Seoul (교통)", &self.seoul_api_key),
            ("Emergency (응급)", &self.emergency_api_key),
        ];

        for (name, key) in &apis {
            if key.is_some() {
                tracing::info!("✅ {} API key configured", name);
            } else {
                tracing::warn!("⚠️  {} API key NOT configured — endpoints will return mock data", name);
            }
        }
    }
}
