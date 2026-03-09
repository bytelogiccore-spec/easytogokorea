use reqwest::Client;

use crate::error::AppError;
use crate::models::transport::*;

pub struct TransportApiClient {
    client: Client,
    service_key: String,
}

impl TransportApiClient {
    pub fn new(client: Client, service_key: String) -> Self {
        Self { client, service_key }
    }

    /// Get bus arrival info for a bus stop (Seoul).
    pub async fn get_bus_arrival(&self, station_id: &str) -> Result<BusArrival, AppError> {
        let url = format!(
            "http://ws.bus.go.kr/api/rest/stationinfo/getStationByUid?ServiceKey={}&arsId={station_id}&resultType=json",
            self.service_key
        );

        let resp = self.client.get(&url).send().await?;
        let root: SeoulBusRoot = resp.json().await?;

        let items = root.msg_body
            .and_then(|b| b.item_list)
            .unwrap_or_default();

        let station_name = items.first()
            .and_then(|i| i.stn_nm.clone())
            .unwrap_or_else(|| station_id.to_string());

        let arrivals = items.into_iter()
            .map(|item| BusArrivalInfo {
                route_name: item.bus_route_nm.unwrap_or_default(),
                route_type: item.route_type.as_deref().map(route_type_name).unwrap_or_default(),
                first_bus: item.arrmsg1,
                second_bus: item.arrmsg2,
                interval: item.term,
            })
            .collect();

        Ok(BusArrival { station_name, arrivals })
    }

    /// Get subway arrival info for a station (Seoul).
    pub async fn get_subway_arrival(&self, station_name: &str) -> Result<SubwayArrival, AppError> {
        let encoded = urlencoding::encode(station_name);
        let url = format!(
            "http://swopenapi.seoul.go.kr/api/subway/{}/json/realtimeStationArrival/0/20/{encoded}",
            self.service_key
        );

        let resp = self.client.get(&url).send().await?;
        let root: SeoulSubwayRoot = resp.json().await?;

        let items = root.realtime_arrival_list.unwrap_or_default();

        let arrivals = items.into_iter()
            .map(|item| SubwayArrivalInfo {
                line: item.train_line_nm.unwrap_or_default(),
                direction: item.updnline.unwrap_or_default(),
                arrival_time_sec: item.barvl_dt.as_deref().and_then(|v| v.parse().ok()),
                message: item.arvl_msg2.unwrap_or_default(),
                current_location: item.arvl_msg3,
            })
            .collect();

        Ok(SubwayArrival {
            station_name: station_name.to_string(),
            arrivals,
        })
    }
}
