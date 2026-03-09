use reqwest::Client;

use crate::error::AppError;
use crate::models::air_quality::*;

const BASE_URL: &str = "https://apis.data.go.kr/B552584/ArpltnInforInqireSvc";

pub struct AirQualityApiClient {
    client: Client,
    service_key: String,
}

impl AirQualityApiClient {
    pub fn new(client: Client, service_key: String) -> Self {
        Self { client, service_key }
    }

    /// Get real-time air quality for a measurement station.
    pub async fn get_realtime(
        &self,
        station_name: &str,
    ) -> Result<AirQuality, AppError> {
        let encoded = urlencoding::encode(station_name);
        let url = format!(
            "{BASE_URL}/getMsrstnAcctoRltmMesureDnsty?ServiceKey={}&stationName={encoded}&dataTerm=DAILY&returnType=json&numOfRows=1&pageNo=1&ver=1.3",
            self.service_key
        );

        let resp = self.client.get(&url).send().await?;
        let root: AirApiRoot = resp.json().await?;

        if root.response.header.result_code != "00" {
            return Err(AppError::ExternalApi(format!(
                "AirKorea API error: {} - {}",
                root.response.header.result_code, root.response.header.result_msg
            )));
        }

        let item = root.response.body
            .and_then(|b| b.items)
            .and_then(|items| items.into_iter().next())
            .ok_or_else(|| AppError::NotFound(format!("No data for station: {station_name}")))?;

        let overall_grade = item.khai_grade.as_deref().unwrap_or("0");
        let (label, emoji, advice) = grade_info(overall_grade);

        let pm10_grade = item.pm10_grade.as_deref().unwrap_or("0");
        let (pm10_label, _, _) = grade_info(pm10_grade);

        let pm25_grade = item.pm25_grade.as_deref().unwrap_or("0");
        let (pm25_label, _, _) = grade_info(pm25_grade);

        Ok(AirQuality {
            measured_at: item.data_time.unwrap_or_default(),
            overall: AqiLevel {
                value: item.khai_value,
                grade: overall_grade.to_string(),
                label,
                emoji,
                advice,
            },
            pm10: PollutantInfo {
                value: item.pm10_value,
                grade: pm10_label,
                label: "PM10".to_string(),
                unit: "㎍/㎥".to_string(),
            },
            pm25: PollutantInfo {
                value: item.pm25_value,
                grade: pm25_label,
                label: "PM2.5".to_string(),
                unit: "㎍/㎥".to_string(),
            },
            ozone: item.o3_value,
            nitrogen_dioxide: item.no2_value,
            carbon_monoxide: item.co_value,
            sulfur_dioxide: item.so2_value,
        })
    }
}
