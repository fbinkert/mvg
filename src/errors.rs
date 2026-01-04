use thiserror::Error;

#[derive(Error, Debug)]
pub enum MvgError {
    #[error("API request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Invalid station ID format")]
    InvalidStationId,
    #[error("Station not found")]
    NotFound,
}
