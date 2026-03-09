use serde::{Deserialize, Serialize};

// ── TourAPI raw response structures ──

#[derive(Debug, Deserialize)]
pub struct TourApiRoot {
    pub response: TourApiResponse,
}

#[derive(Debug, Deserialize)]
pub struct TourApiResponse {
    pub header: TourApiHeader,
    pub body: Option<TourApiBody>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TourApiHeader {
    pub result_code: String,
    pub result_msg: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TourApiBody {
    pub items: Option<TourApiItems>,
    pub num_of_rows: Option<i32>,
    pub page_no: Option<i32>,
    pub total_count: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct TourApiItems {
    pub item: Option<TourApiItemList>,
}

/// TourAPI returns either a single object or an array depending on count.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TourApiItemList {
    Single(Box<TourApiRawItem>),
    Multiple(Vec<TourApiRawItem>),
}

impl TourApiItemList {
    pub fn into_vec(self) -> Vec<TourApiRawItem> {
        match self {
            TourApiItemList::Single(item) => vec![*item],
            TourApiItemList::Multiple(items) => items,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TourApiRawItem {
    pub contentid: Option<String>,
    pub contenttypeid: Option<String>,
    pub title: Option<String>,
    pub addr1: Option<String>,
    pub addr2: Option<String>,
    pub mapx: Option<String>,
    pub mapy: Option<String>,
    pub firstimage: Option<String>,
    pub firstimage2: Option<String>,
    pub tel: Option<String>,
    pub dist: Option<String>,
    pub areacode: Option<String>,
    pub sigungucode: Option<String>,
    pub cat1: Option<String>,
    pub cat2: Option<String>,
    pub cat3: Option<String>,
    pub overview: Option<String>,
}

// ── Transformed output models ──

#[derive(Debug, Serialize, Clone)]
pub struct Attraction {
    pub id: String,
    pub name: String,
    pub content_type: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
}

impl From<TourApiRawItem> for Attraction {
    fn from(item: TourApiRawItem) -> Self {
        Self {
            id: item.contentid.unwrap_or_default(),
            name: item.title.unwrap_or_default(),
            content_type: content_type_name(item.contenttypeid.as_deref().unwrap_or("")),
            address: format!(
                "{}{}",
                item.addr1.unwrap_or_default(),
                item.addr2.map(|a| format!(" {a}")).unwrap_or_default()
            ),
            lat: item.mapy.as_deref().and_then(|v| v.parse().ok()),
            lng: item.mapx.as_deref().and_then(|v| v.parse().ok()),
            thumbnail: item.firstimage,
            tel: item.tel,
            distance: item.dist.as_deref().and_then(|v| v.parse().ok()),
            overview: item.overview,
        }
    }
}

fn content_type_name(code: &str) -> String {
    match code {
        "12" => "관광지".to_string(),
        "14" => "문화시설".to_string(),
        "15" => "축제/행사".to_string(),
        "25" => "여행코스".to_string(),
        "28" => "레포츠".to_string(),
        "32" => "숙박".to_string(),
        "38" => "쇼핑".to_string(),
        "39" => "음식점".to_string(),
        _ => code.to_string(),
    }
}

#[derive(Debug, Serialize)]
pub struct AttractionList {
    pub items: Vec<Attraction>,
    pub total_count: i32,
    pub page: i32,
}
