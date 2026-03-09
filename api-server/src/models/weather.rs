use serde::{Deserialize, Serialize};

// ── 기상청 API raw response ──

#[derive(Debug, Deserialize)]
pub struct WeatherApiRoot {
    pub response: WeatherApiResponse,
}

#[derive(Debug, Deserialize)]
pub struct WeatherApiResponse {
    pub header: WeatherApiHeader,
    pub body: Option<WeatherApiBody>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherApiHeader {
    pub result_code: String,
    pub result_msg: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherApiBody {
    pub data_type: Option<String>,
    pub items: Option<WeatherApiItems>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherApiItems {
    pub item: Option<Vec<WeatherRawItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherRawItem {
    pub base_date: Option<String>,
    pub base_time: Option<String>,
    pub category: Option<String>,
    pub fcst_date: Option<String>,
    pub fcst_time: Option<String>,
    pub fcst_value: Option<String>,
    pub nx: Option<i32>,
    pub ny: Option<i32>,
}

// ── Transformed output ──

#[derive(Debug, Serialize, Clone)]
pub struct WeatherForecast {
    pub date: String,
    pub time: String,
    pub temperature: Option<String>,
    pub sky: Option<WeatherCondition>,
    pub precipitation_type: Option<PrecipitationType>,
    pub humidity: Option<String>,
    pub wind_speed: Option<String>,
    pub wind_direction: Option<String>,
    pub precipitation_probability: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct WeatherCondition {
    pub code: String,
    pub label: String,
    pub emoji: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PrecipitationType {
    pub code: String,
    pub label: String,
    pub emoji: String,
}

pub fn sky_condition(code: &str) -> WeatherCondition {
    match code {
        "1" => WeatherCondition { code: "1".into(), label: "Clear".into(), emoji: "☀️".into() },
        "3" => WeatherCondition { code: "3".into(), label: "Mostly Cloudy".into(), emoji: "⛅".into() },
        "4" => WeatherCondition { code: "4".into(), label: "Cloudy".into(), emoji: "☁️".into() },
        _ => WeatherCondition { code: code.into(), label: "Unknown".into(), emoji: "❓".into() },
    }
}

pub fn precipitation_type(code: &str) -> PrecipitationType {
    match code {
        "0" => PrecipitationType { code: "0".into(), label: "None".into(), emoji: "☀️".into() },
        "1" => PrecipitationType { code: "1".into(), label: "Rain".into(), emoji: "🌧️".into() },
        "2" => PrecipitationType { code: "2".into(), label: "Rain/Snow".into(), emoji: "🌨️".into() },
        "3" => PrecipitationType { code: "3".into(), label: "Snow".into(), emoji: "❄️".into() },
        "4" => PrecipitationType { code: "4".into(), label: "Shower".into(), emoji: "🌦️".into() },
        _ => PrecipitationType { code: code.into(), label: "Unknown".into(), emoji: "❓".into() },
    }
}
