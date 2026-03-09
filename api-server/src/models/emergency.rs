use serde::{Deserialize, Serialize};

// ── Emergency API raw response ──

#[derive(Debug, Deserialize)]
pub struct EmergencyApiRoot {
    pub response: EmergencyApiResponse,
}

#[derive(Debug, Deserialize)]
pub struct EmergencyApiResponse {
    pub header: EmergencyApiHeader,
    pub body: Option<EmergencyApiBody>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyApiHeader {
    pub result_code: String,
    pub result_msg: String,
}

#[derive(Debug, Deserialize)]
pub struct EmergencyApiBody {
    pub items: Option<EmergencyApiItems>,
}

#[derive(Debug, Deserialize)]
pub struct EmergencyApiItems {
    pub item: Option<Vec<EmergencyRawItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyRawItem {
    pub duty_name: Option<String>,       // 기관명
    pub duty_addr: Option<String>,       // 주소
    pub duty_tel1: Option<String>,       // 대표전화
    pub duty_tel3: Option<String>,       // 응급실 전화
    pub wgs84_lon: Option<f64>,          // 경도
    pub wgs84_lat: Option<f64>,          // 위도
    pub hvec: Option<i32>,               // 응급실 가용 병상
    pub hv_s01: Option<i32>,             // 일반 입원실
    pub hvidate: Option<String>,         // 데이터 기준 시각
    pub duty_div_nam: Option<String>,    // 기관 분류명
}

// ── Transformed output ──

#[derive(Debug, Serialize, Clone)]
pub struct EmergencyRoom {
    pub name: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_beds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

impl From<EmergencyRawItem> for EmergencyRoom {
    fn from(item: EmergencyRawItem) -> Self {
        Self {
            name: item.duty_name.unwrap_or_default(),
            address: item.duty_addr.unwrap_or_default(),
            phone: item.duty_tel1,
            emergency_phone: item.duty_tel3,
            lat: item.wgs84_lat,
            lng: item.wgs84_lon,
            available_beds: item.hvec,
            category: item.duty_div_nam,
            last_updated: item.hvidate,
        }
    }
}
