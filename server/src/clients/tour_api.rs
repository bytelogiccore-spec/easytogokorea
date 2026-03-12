use reqwest::Client;

use crate::error::AppError;
use crate::models::tour::*;

const BASE_URL: &str = "https://apis.data.go.kr/B551011/KorService1";

pub struct TourApiClient {
    client: Client,
    service_key: String,
}

impl TourApiClient {
    pub fn new(client: Client, service_key: String) -> Self {
        Self { client, service_key }
    }

    /// Search nearby attractions by GPS coordinates.
    pub async fn get_nearby(
        &self,
        lat: f64,
        lng: f64,
        radius: i32,
        content_type_id: Option<&str>,
        page: i32,
        num_of_rows: i32,
    ) -> Result<AttractionList, AppError> {
        let mut url = format!(
            "{BASE_URL}/locationBasedList1?ServiceKey={}&MobileOS=ETC&MobileApp=EasyToGoKorea&_type=json&mapX={lng}&mapY={lat}&radius={radius}&numOfRows={num_of_rows}&pageNo={page}",
            self.service_key
        );
        if let Some(ct) = content_type_id {
            url.push_str(&format!("&contentTypeId={ct}"));
        }

        let resp = self.client.get(&url).send().await?;
        let root: TourApiRoot = resp.json().await?;
        self.parse_response(root, page)
    }

    /// Search attractions by keyword.
    pub async fn search(
        &self,
        keyword: &str,
        content_type_id: Option<&str>,
        page: i32,
        num_of_rows: i32,
    ) -> Result<AttractionList, AppError> {
        let encoded_keyword = urlencoding::encode(keyword);
        let mut url = format!(
            "{BASE_URL}/searchKeyword1?ServiceKey={}&MobileOS=ETC&MobileApp=EasyToGoKorea&_type=json&keyword={encoded_keyword}&numOfRows={num_of_rows}&pageNo={page}",
            self.service_key
        );
        if let Some(ct) = content_type_id {
            url.push_str(&format!("&contentTypeId={ct}"));
        }

        let resp = self.client.get(&url).send().await?;
        let root: TourApiRoot = resp.json().await?;
        self.parse_response(root, page)
    }

    /// Get detail info for a content item.
    pub async fn get_detail(
        &self,
        content_id: &str,
        content_type_id: Option<&str>,
    ) -> Result<Attraction, AppError> {
        let mut url = format!(
            "{BASE_URL}/detailCommon1?ServiceKey={}&MobileOS=ETC&MobileApp=EasyToGoKorea&_type=json&contentId={content_id}&defaultYN=Y&overviewYN=Y&firstImageYN=Y&addrinfoYN=Y&mapinfoYN=Y",
            self.service_key
        );
        if let Some(ct) = content_type_id {
            url.push_str(&format!("&contentTypeId={ct}"));
        }

        let resp = self.client.get(&url).send().await?;
        let root: TourApiRoot = resp.json().await?;

        let items = root.response.body
            .and_then(|b| b.items)
            .and_then(|i| i.item)
            .map(|il| il.into_vec())
            .unwrap_or_default();

        items.into_iter().next()
            .map(Attraction::from)
            .ok_or_else(|| AppError::NotFound(format!("Content {content_id} not found")))
    }

    /// Search festivals/events.
    pub async fn get_festivals(
        &self,
        event_start_date: &str,
        page: i32,
        num_of_rows: i32,
    ) -> Result<AttractionList, AppError> {
        let url = format!(
            "{BASE_URL}/searchFestival1?ServiceKey={}&MobileOS=ETC&MobileApp=EasyToGoKorea&_type=json&eventStartDate={event_start_date}&numOfRows={num_of_rows}&pageNo={page}",
            self.service_key
        );

        let resp = self.client.get(&url).send().await?;
        let root: TourApiRoot = resp.json().await?;
        self.parse_response(root, page)
    }

    fn parse_response(&self, root: TourApiRoot, page: i32) -> Result<AttractionList, AppError> {
        if root.response.header.result_code != "0000" {
            return Err(AppError::ExternalApi(format!(
                "TourAPI error: {} - {}",
                root.response.header.result_code, root.response.header.result_msg
            )));
        }

        let total_count = root.response.body.as_ref()
            .and_then(|b| b.total_count)
            .unwrap_or(0);

        let items = root.response.body
            .and_then(|b| b.items)
            .and_then(|i| i.item)
            .map(|il| il.into_vec())
            .unwrap_or_default()
            .into_iter()
            .map(Attraction::from)
            .collect();

        Ok(AttractionList { items, total_count, page })
    }
}
