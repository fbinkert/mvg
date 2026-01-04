use std::sync::OnceLock;

use regex::Regex;
use reqwest::{Client, Url};

use crate::{
    errors::MvgError,
    models::{Departure, Line, Station, TransportType, ZdmStation},
};

pub struct MvgClient {
    client: Client,
    fib_base: Url,
    zdm_base: Url,
}

const MVG_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)";
const MVG_ZDM_BASE: &str = "https://www.mvg.de/.rest/zdm/";
const MVG_FIB_BASE: &str = "https://www.mvg.de/api/bgw-pt/v3/";

impl Default for MvgClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MvgClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(MVG_USER_AGENT)
            .build()
            .unwrap();
        Self {
            client,
            fib_base: Url::parse(MVG_FIB_BASE).unwrap(),
            zdm_base: Url::parse(MVG_ZDM_BASE).unwrap(),
        }
    }

    fn is_valid_station_id(id: &str) -> bool {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"de:[0-9]{2,5}:[0-9]+").unwrap());
        re.is_match(id)
    }

    /// Find a station by global ID
    pub async fn get_station_by_id(&self, id: &str) -> Result<Station, MvgError> {
        if !Self::is_valid_station_id(id) {
            return Err(MvgError::InvalidStationId);
        }
        let url = self.zdm_base.join(&format!("stations/{}", id)).unwrap();
        let resp = self.client.get(url).send().await?;
        if resp.status().is_success() {
            let station: Station = resp.json().await?;
            return Ok(station);
        }
        Err(MvgError::NotFound)
    }

    /// Find stations by name
    pub async fn get_stations_by_name(&self, name: &str) -> Result<Vec<Station>, MvgError> {
        let mut url = self.fib_base.join("locations").unwrap();
        url.query_pairs_mut()
            .append_pair("query", name)
            .append_pair("locationTypes", "STATION");
        let resp = self.client.get(url).send().await?.error_for_status()?;
        let stations: Vec<Station> = resp.json().await?;
        Ok(stations)
    }

    /// Find a station by name. The first match is returned.
    pub async fn get_station_by_name(&self, name: &str) -> Result<Option<Station>, MvgError> {
        let stations = self.get_stations_by_name(name).await?;
        Ok(stations.into_iter().next())
    }

    /// Retrieve departures for a specific station ID
    pub async fn get_departures(
        &self,
        station_id: &str,
        limit: usize,
        offset_mins: i32,
        filters: Option<Vec<TransportType>>,
    ) -> Result<Vec<Departure>, MvgError> {
        if !Self::is_valid_station_id(station_id) {
            return Err(MvgError::InvalidStationId);
        }

        let mut url = self.fib_base.join("departures").unwrap();
        {
            let mut query = url.query_pairs_mut();
            query
                .append_pair("globalId", station_id)
                .append_pair("limit", &limit.to_string())
                .append_pair("offsetInMinutes", &offset_mins.to_string());

            if let Some(f) = filters {
                let types: Vec<String> = f
                    .iter()
                    .map(|t| format!("{:?}", t).to_uppercase())
                    .collect();
                query.append_pair("transportTypes", &types.join(","));
            }
        }

        let departures: Vec<Departure> = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(departures)
    }

    /// Find stations near GPS coordinates
    pub async fn nearby(&self, lat: f64, lon: f64) -> Result<Vec<Station>, MvgError> {
        let mut url = self.fib_base.join("stations/nearby").unwrap();
        url.query_pairs_mut()
            .append_pair("latitude", &lat.to_string())
            .append_pair("longitude", &lon.to_string());

        let stations: Vec<Station> = self.client.get(url).send().await?.json().await?;
        Ok(stations)
    }

    /// Retrieves the full list of all stations from the ZDM master database.
    pub async fn list_stations(&self) -> Result<Vec<ZdmStation>, MvgError> {
        let url = self.zdm_base.join("stations").unwrap();
        let response = self.client.get(url).send().await?.error_for_status()?;
        let stations: Vec<ZdmStation> = response.json().await?;
        Ok(stations)
    }

    /// Retrieve a list of all lines from the ZDM API
    pub async fn get_lines(&self) -> Result<Vec<Line>, MvgError> {
        let url = self.zdm_base.join("lines").unwrap();
        let resp = self.client.get(url).send().await?.error_for_status()?;
        let lines: Vec<Line> = resp.json().await?;
        Ok(lines)
    }

    /// Retrieve a list of all station global IDs
    pub async fn get_station_global_ids(&self) -> Result<Vec<String>, MvgError> {
        let url = self.zdm_base.join("mvgStationGlobalIds").unwrap();
        let resp = self.client.get(url).send().await?.error_for_status()?;
        let ids: Vec<String> = resp.json().await?;
        Ok(ids)
    }
}
