use reqwest::Client;

use crate::error::AppError;
use crate::models::exchange::*;

pub struct ExchangeApiClient {
    client: Client,
    api_key: String,
}

impl ExchangeApiClient {
    pub fn new(client: Client, api_key: String) -> Self {
        Self { client, api_key }
    }

    /// Get exchange rates from Bank of Korea.
    /// stat_code 731Y001: Principal exchange rates
    pub async fn get_rates(&self, date: &str) -> Result<ExchangeRateList, AppError> {
        let url = format!(
            "https://ecos.bok.or.kr/api/StatisticSearch/{}/json/kr/1/20/731Y001/D/{date}/{date}/0000001",
            self.api_key
        );

        let resp = self.client.get(&url).send().await?;
        let text = resp.text().await?;
        let root: BokApiRoot = serde_json::from_str(&text)
            .map_err(|e| AppError::ExternalApi(format!("BOK API parse error: {e} — body: {}", &text[..text.len().min(200)])))?;

        let rows = root.statistic_search
            .and_then(|s| s.row)
            .unwrap_or_default();

        let rates: Vec<ExchangeRate> = rows.into_iter()
            .filter_map(|row| {
                let currency_name = row.item_name1.unwrap_or_default();
                let rate_str = row.data_value.unwrap_or_default();
                let rate: f64 = rate_str.replace(',', "").parse().ok()?;
                let unit = row.unit_name.unwrap_or_else(|| "원".to_string());
                let date = row.time.unwrap_or_default();
                let flag = currency_flag(&currency_name);

                Some(ExchangeRate {
                    currency: currency_name.clone(),
                    currency_name,
                    rate,
                    unit,
                    date: date.clone(),
                    flag_emoji: flag.to_string(),
                })
            })
            .collect();

        let last_updated = rates.first().map(|r| r.date.clone()).unwrap_or_default();

        Ok(ExchangeRateList {
            base: "KRW".to_string(),
            rates,
            last_updated,
        })
    }
}
