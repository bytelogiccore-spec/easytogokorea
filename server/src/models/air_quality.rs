use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

// ── AirKorea API raw response ──

#[derive(Debug, Deserialize)]
pub struct AirApiRoot {
    pub response: AirApiResponse,
}

#[derive(Debug, Deserialize)]
pub struct AirApiResponse {
    pub header: AirApiHeader,
    pub body: Option<AirApiBody>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirApiHeader {
    pub result_code: String,
    pub result_msg: String,
}

#[derive(Debug, Deserialize)]
pub struct AirApiBody {
    pub items: Option<Vec<AirRawItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirRawItem {
    pub data_time: Option<String>,
    pub so2_value: Option<String>,
    pub co_value: Option<String>,
    pub o3_value: Option<String>,
    pub no2_value: Option<String>,
    pub pm10_value: Option<String>,
    pub pm25_value: Option<String>,
    pub khai_value: Option<String>,
    pub khai_grade: Option<String>,
    pub pm10_grade: Option<String>,
    pub pm25_grade: Option<String>,
}

// ── Transformed output ──

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct AirQuality {
    pub measured_at: String,
    pub overall: AqiLevel,
    pub pm10: PollutantInfo,
    pub pm25: PollutantInfo,
    pub ozone: Option<String>,
    pub nitrogen_dioxide: Option<String>,
    pub carbon_monoxide: Option<String>,
    pub sulfur_dioxide: Option<String>,
}

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct AqiLevel {
    pub value: Option<String>,
    pub grade: String,
    pub label: String,
    pub emoji: String,
    pub advice: String,
}

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct PollutantInfo {
    pub value: Option<String>,
    pub grade: String,
    pub label: String,
    pub unit: String,
}

pub fn grade_info(grade: &str) -> (String, String, String) {
    match grade {
        "1" => ("Good".into(), "🟢".into(), "Perfect for outdoor activities".into()),
        "2" => ("Moderate".into(), "🟡".into(), "Sensitive groups should be cautious".into()),
        "3" => ("Unhealthy".into(), "🟠".into(), "Reduce outdoor activities".into()),
        "4" => ("Very Unhealthy".into(), "🔴".into(), "Stay indoors".into()),
        _ => ("Unknown".into(), "⚪".into(), "No data available".into()),
    }
}
