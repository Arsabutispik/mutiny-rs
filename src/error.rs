use thiserror::Error;
use crate::http::HttpError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid channel type: {0}")]
    InvalidChannelType(String),

    #[error("HTTP request failed: {0}")]
    Http(#[from] HttpError),

    #[error("Failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API Error: {0}")]
    Api(String),
}