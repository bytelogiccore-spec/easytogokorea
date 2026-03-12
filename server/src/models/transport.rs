use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

// ── Seoul Bus API raw response ──

#[derive(Debug, Deserialize)]
pub struct SeoulBusRoot {
    #[serde(rename = "msgBody")]
    pub msg_body: Option<SeoulBusMsgBody>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeoulBusMsgBody {
    pub item_list: Option<Vec<SeoulBusRawItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SeoulBusRawItem {
    pub stn_nm: Option<String>,        // 정류장명
    pub bus_route_nm: Option<String>,   // 버스 노선명
    pub arrmsg1: Option<String>,        // 첫번째 도착 메시지
    pub arrmsg2: Option<String>,        // 두번째 도착 메시지
    pub route_type: Option<String>,     // 노선 유형
    pub next_bus: Option<String>,
    pub term: Option<String>,           // 배차 간격
}

// ── Seoul Subway API raw response ──

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeoulSubwayRoot {
    pub realtime_arrival_list: Option<Vec<SubwayRawItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct SubwayRawItem {
    pub subway_id: Option<String>,
    pub updnline: Option<String>,       // 상행/하행
    pub train_line_nm: Option<String>,  // 노선명
    pub statn_nm: Option<String>,       // 역명
    pub barvl_dt: Option<String>,       // 도착 예정 시간 (초)
    pub arvl_msg2: Option<String>,      // 도착 메시지
    pub arvl_msg3: Option<String>,      // 현재 위치
    pub arvl_cd: Option<String>,        // 도착 코드
}

// ── Transformed output ──

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct BusArrival {
    pub station_name: String,
    pub arrivals: Vec<BusArrivalInfo>,
}

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct BusArrivalInfo {
    pub route_name: String,
    pub route_type: String,
    pub first_bus: Option<String>,
    pub second_bus: Option<String>,
    pub interval: Option<String>,
}

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct SubwayArrival {
    pub station_name: String,
    pub arrivals: Vec<SubwayArrivalInfo>,
}

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct SubwayArrivalInfo {
    pub line: String,
    pub direction: String,
    pub arrival_time_sec: Option<i32>,
    pub message: String,
    pub current_location: Option<String>,
}

pub fn route_type_name(code: &str) -> String {
    match code {
        "1" => "공항".into(),
        "2" => "마을".into(),
        "3" => "간선".into(),
        "4" => "지선".into(),
        "5" => "순환".into(),
        "6" => "광역".into(),
        "7" => "인천".into(),
        "8" => "경기".into(),
        "9" => "폐지".into(),
        _ => code.to_string(),
    }
}
