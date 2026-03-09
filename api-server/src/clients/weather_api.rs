use std::collections::HashMap;

use reqwest::Client;

use crate::error::AppError;
use crate::models::weather::*;

const BASE_URL: &str = "https://apis.data.go.kr/1360000/VilageFcstInfoService_2.0";

pub struct WeatherApiClient {
    client: Client,
    service_key: String,
}

impl WeatherApiClient {
    pub fn new(client: Client, service_key: String) -> Self {
        Self { client, service_key }
    }

    /// Get short-term forecast (3 days).
    pub async fn get_forecast(
        &self,
        nx: i32,
        ny: i32,
        base_date: &str,
        base_time: &str,
    ) -> Result<Vec<WeatherForecast>, AppError> {
        let url = format!(
            "{BASE_URL}/getVilageFcst?ServiceKey={}&pageNo=1&numOfRows=1000&dataType=JSON&base_date={base_date}&base_time={base_time}&nx={nx}&ny={ny}",
            self.service_key
        );

        let resp = self.client.get(&url).send().await?;
        let root: WeatherApiRoot = resp.json().await?;

        if root.response.header.result_code != "00" {
            return Err(AppError::ExternalApi(format!(
                "Weather API error: {} - {}",
                root.response.header.result_code, root.response.header.result_msg
            )));
        }

        let items = root.response.body
            .and_then(|b| b.items)
            .and_then(|i| i.item)
            .unwrap_or_default();

        Ok(self.group_forecast(items))
    }

    /// Get ultra-short-term observation (current weather).
    pub async fn get_current(
        &self,
        nx: i32,
        ny: i32,
        base_date: &str,
        base_time: &str,
    ) -> Result<Vec<WeatherForecast>, AppError> {
        let url = format!(
            "{BASE_URL}/getUltraSrtNcst?ServiceKey={}&pageNo=1&numOfRows=100&dataType=JSON&base_date={base_date}&base_time={base_time}&nx={nx}&ny={ny}",
            self.service_key
        );

        let resp = self.client.get(&url).send().await?;
        let root: WeatherApiRoot = resp.json().await?;

        if root.response.header.result_code != "00" {
            return Err(AppError::ExternalApi(format!(
                "Weather API error: {} - {}",
                root.response.header.result_code, root.response.header.result_msg
            )));
        }

        let items = root.response.body
            .and_then(|b| b.items)
            .and_then(|i| i.item)
            .unwrap_or_default();

        Ok(self.group_forecast(items))
    }

    /// Group raw weather items by (date, time) into forecasts.
    fn group_forecast(&self, items: Vec<WeatherRawItem>) -> Vec<WeatherForecast> {
        // Group by (fcst_date, fcst_time) or (base_date, base_time)
        let mut grouped: HashMap<(String, String), HashMap<String, String>> = HashMap::new();

        for item in items {
            let date = item.fcst_date.or(item.base_date).unwrap_or_default();
            let time = item.fcst_time.or(item.base_time).unwrap_or_default();
            let category = item.category.unwrap_or_default();
            let value = item.fcst_value.unwrap_or_default();

            grouped.entry((date, time))
                .or_default()
                .insert(category, value);
        }

        let mut forecasts: Vec<WeatherForecast> = grouped.into_iter()
            .map(|((date, time), cats)| {
                WeatherForecast {
                    date,
                    time,
                    temperature: cats.get("TMP").or(cats.get("T1H")).cloned(),
                    sky: cats.get("SKY").map(|v| sky_condition(v)),
                    precipitation_type: cats.get("PTY").map(|v| precipitation_type(v)),
                    humidity: cats.get("REH").cloned(),
                    wind_speed: cats.get("WSD").cloned(),
                    wind_direction: cats.get("VEC").cloned(),
                    precipitation_probability: cats.get("POP").cloned(),
                }
            })
            .collect();

        forecasts.sort_by(|a, b| (&a.date, &a.time).cmp(&(&b.date, &b.time)));
        forecasts
    }
}
