use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}
