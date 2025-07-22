use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not Found")]
    NotFound,
    #[error("Server Error")]
    ServerError,
    #[error("Forbidden")]
    Forbidden,
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Other error: {0}")]
    Other(String),
}
#[derive(Clone)]
pub struct Http {
    client: reqwest::Client,
    base_url: String,
    token: String,
}
pub const BASE_URL: &str = "https://api.revolt.chat";
impl Http {
    pub fn new(token: String) -> Self {
        Http {
            client: reqwest::Client::new(),
            base_url: BASE_URL.to_string(),
            token,
        }
    }
    fn status_to_error(status: reqwest::StatusCode) -> HttpError {
        match status {
            reqwest::StatusCode::UNAUTHORIZED => HttpError::Unauthorized,
            reqwest::StatusCode::NOT_FOUND => HttpError::NotFound,
            reqwest::StatusCode::FORBIDDEN => HttpError::Forbidden,
            s if s.is_server_error() => HttpError::ServerError,
            _ => HttpError::Other(format!("Unexpected status code: {}", status)),
        }
    }
    pub async fn post<T: DeserializeOwned>(&self, path: &str, body: &Value) -> Result<T, HttpError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self.client
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header("x-bot-token", &self.token)
            .json(body)
            .send()
            .await
            .map_err(HttpError::from)?;

        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|e| HttpError::Other(format!("Failed to read body: {}", e)))?;

        Self::status_to_error(status);

        let parsed = serde_json::from_str::<T>(&text)
            .map_err(|e| HttpError::Other(format!("Failed to parse JSON: {}. Body: {}", e, text)))?;

        Ok(parsed)
    }
    pub async fn post_empty(&self, path: &str) -> Result<(), HttpError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self.client
            .post(&url)
            .header("x-bot-token", &self.token)
            .send()
            .await
            .map_err(HttpError::from)?;

        let status = response.status();

        Self::status_to_error(status);

        Ok(())
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, HttpError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self.client
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("x-bot-token", &self.token)
            .send()
            .await
            .map_err(HttpError::from)?;

        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|e| HttpError::Other(format!("Failed to read body: {}", e)))?;

        Self::status_to_error(status);

        let parsed = serde_json::from_str::<T>(&text)
            .map_err(|e| HttpError::Other(format!("Failed to parse JSON: {}. Body: {}", e, text)))?;

        Ok(parsed)
    }
    pub async fn delete(&self, path: &str) -> Result<(), HttpError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self.client
            .delete(&url)
            .header("x-bot-token", &self.token)
            .send()
            .await
            .map_err(HttpError::from)?;

        let status = response.status();
        Self::status_to_error(status);

        Ok(())
    }

    pub async fn patch<T: DeserializeOwned>(&self, path: &str, body: &Value) -> Result<T, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client
            .patch(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header("x-bot-token", &self.token)
            .json(body)
            .send()
            .await
            .map_err(HttpError::from)?;

        let status = response.status();

        let text = response
            .text()
            .await
            .map_err(|e| HttpError::Other(format!("Failed to read body: {}", e)))?;

        Self::status_to_error(status);

        let parsed = serde_json::from_str::<T>(&text)
            .map_err(|e| HttpError::Other(format!("Failed to parse JSON: {}. Body: {}", e, text)))?;

        Ok(parsed)
    }
}
