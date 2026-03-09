use reqwest::Client;

use crate::error::AppError;
use crate::models::emergency::*;

const BASE_URL: &str = "https://apis.data.go.kr/B552657/ErmctInfoInqireService";

pub struct EmergencyApiClient {
    client: Client,
    service_key: String,
}

impl EmergencyApiClient {
    pub fn new(client: Client, service_key: String) -> Self {
        Self { client, service_key }
    }

    /// Get real-time emergency room availability.
    pub async fn get_emergency_rooms(
        &self,
        stage1: &str,
        stage2: Option<&str>,
    ) -> Result<Vec<EmergencyRoom>, AppError> {
        let encoded_stage1 = urlencoding::encode(stage1);
        let mut url = format!(
            "{BASE_URL}/getEmrrmRltmUsefulSckbdInfoInqire?ServiceKey={}&STAGE1={encoded_stage1}&pageNo=1&numOfRows=20",
            self.service_key
        );

        if let Some(s2) = stage2 {
            url.push_str(&format!("&STAGE2={}", urlencoding::encode(s2)));
        }

        let resp = self.client.get(&url).send().await?;
        let root: EmergencyApiRoot = resp.json().await?;

        if root.response.header.result_code != "00" {
            return Err(AppError::ExternalApi(format!(
                "Emergency API error: {} - {}",
                root.response.header.result_code, root.response.header.result_msg
            )));
        }

        let items = root.response.body
            .and_then(|b| b.items)
            .and_then(|i| i.item)
            .unwrap_or_default();

        Ok(items.into_iter().map(EmergencyRoom::from).collect())
    }
}
