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

impl MvgClient {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(MVG_USER_AGENT),
        );

        Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
            fib_base: Url::parse(MVG_FIB_BASE).unwrap(),
            zdm_base: Url::parse(MVG_ZDM_BASE).unwrap(),
        }
    }

    pub fn is_valid_station_id(id: &str) -> bool {
        let re = Regex::new(r"de:[0-9]{2,5}:[0-9]+").unwrap();
        re.is_match(id)
    }

    /// Find a station by name/query or global ID
    pub async fn get_station(&self, query: &str) -> Result<Option<Station>, MvgError> {
        if Self::is_valid_station_id(query) {
            // --- CASE A: ZDM Single Station ---
            // Endpoint: /.rest/zdm/stations/{id} -> Returns a MAP {}
            let url = self.zdm_base.join(&format!("stations/{}", query)).unwrap();
            let resp = self.client.get(url).send().await?;

            if resp.status().is_success() {
                let station: Station = resp.json().await?; // Expects a Map
                return Ok(Some(station));
            }
            return Ok(None);
        }

        // --- CASE B: FIB Location Search ---
        // Endpoint: /api/bgw-pt/v3/locations?query=... -> Returns a SEQUENCE [...]
        let mut url = self.fib_base.join("locations").unwrap();
        url.query_pairs_mut()
            .append_pair("query", query)
            .append_pair("locationTypes", "STATION");

        let resp = self.client.get(url).send().await?;
        let stations: Vec<Station> = resp.json().await?; // Expects a Sequence
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

        let departures: Vec<Departure> = self.client.get(url).send().await?.json().await?;
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

        let response = self.client.get(url).send().await?;

        println!("{:?}", response);
        if !response.status().is_success() {
            return Err(MvgError::NotFound); // Or a specific status error
        }

        let stations: Vec<ZdmStation> = response.json().await?;
        Ok(stations)
    }

    /// Retrieve a list of all lines from the ZDM API
    pub async fn get_lines(&self) -> Result<Vec<Line>, MvgError> {
        let url = self.zdm_base.join("lines").unwrap();
        let resp = self.client.get(url).send().await?;

        if !resp.status().is_success() {
            return Err(MvgError::NotFound);
        }

        let lines: Vec<Line> = resp.json().await?;
        Ok(lines)
    }

    /// Retrieve a list of all station global IDs
    pub async fn get_station_global_ids(&self) -> Result<Vec<String>, MvgError> {
        let url = self.zdm_base.join("mvgStationGlobalIds").unwrap();
        let resp = self.client.get(url).send().await?;

        if !resp.status().is_success() {
            return Err(MvgError::NotFound);
        }

        let ids: Vec<String> = resp.json().await?;
        Ok(ids)
    }
}
