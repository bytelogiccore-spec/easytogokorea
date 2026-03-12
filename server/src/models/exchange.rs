use serde::{Deserialize, Serialize};
use async_graphql::SimpleObject;

// ── BOK API raw response ──

#[derive(Debug, Deserialize)]
pub struct BokApiRoot {
    #[serde(rename = "StatisticSearch")]
    pub statistic_search: Option<BokStatisticSearch>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BokStatisticSearch {
    pub list_total_count: Option<i32>,
    pub row: Option<Vec<BokRawRow>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)]
pub struct BokRawRow {
    pub stat_name: Option<String>,
    pub item_name1: Option<String>,
    pub data_value: Option<String>,
    pub time: Option<String>,
    pub unit_name: Option<String>,
}

// ── Transformed output ──

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct ExchangeRate {
    pub currency: String,
    pub currency_name: String,
    pub rate: f64,
    pub unit: String,
    pub date: String,
    pub flag_emoji: String,
}

#[derive(Debug, Serialize, Clone, SimpleObject)]
pub struct ExchangeRateList {
    pub base: String,
    pub rates: Vec<ExchangeRate>,
    pub last_updated: String,
}

pub fn currency_flag(currency: &str) -> &'static str {
    match currency {
        "USD" | "미국 달러" => "🇺🇸",
        "JPY" | "일본 엔" => "🇯🇵",
        "EUR" | "유로" => "🇪🇺",
        "GBP" | "영국 파운드" => "🇬🇧",
        "CNY" | "중국 위안" => "🇨🇳",
        "THB" | "태국 바트" => "🇹🇭",
        "VND" | "베트남 동" => "🇻🇳",
        "AUD" | "호주 달러" => "🇦🇺",
        "CAD" | "캐나다 달러" => "🇨🇦",
        _ => "🏳️",
    }
}
