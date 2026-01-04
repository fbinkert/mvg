use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransportType {
    Bahn,
    Sbahn,
    Ubahn,
    Tram,
    Bus,
    RegionalBus,
    Sev,
    Schiff,
}

impl TransportType {
    pub fn label(&self) -> &str {
        match self {
            Self::Bahn => "Bahn",
            Self::Sbahn => "S-Bahn",
            Self::Ubahn => "U-Bahn",
            Self::Tram => "Tram",
            Self::Bus => "Bus",
            Self::RegionalBus => "Regionalbus",
            Self::Sev => "SEV",
            Self::Schiff => "Schiff",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::Bahn => "mdi:train",
            Self::Sbahn => "mdi:subway-variant",
            Self::Ubahn => "mdi:subway",
            Self::Tram => "mdi:tram",
            _ => "mdi:bus",
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    #[serde(alias = "globalId")]
    pub id: String,
    pub abbreviation: Option<String>,
    pub diva_id: Option<u32>,
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
    pub place: String,
    pub products: Option<Vec<String>>,
    pub tariff_zones: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ZdmStation {
    pub name: String,
    pub place: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    #[serde(rename = "realtimeDepartureTime")]
    pub time_ms: i64,
    #[serde(rename = "plannedDepartureTime")]
    pub planned_ms: i64,
    pub delay_in_minutes: Option<i32>,
    pub platform: Option<i32>,
    pub realtime: bool,
    pub label: String,
    pub destination: String,
    pub transport_type: TransportType,
    pub cancelled: bool,
    pub messages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub line_number: Option<i32>,
    pub name: Option<String>,
    pub product: Option<String>,
}
