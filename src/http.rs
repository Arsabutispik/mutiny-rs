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
    fn status(status: reqwest::StatusCode) -> Result<(), HttpError> {
        match status {
            reqwest::StatusCode::UNAUTHORIZED => Err(HttpError::Unauthorized),
            reqwest::StatusCode::NOT_FOUND => Err(HttpError::NotFound),
            s if s.is_server_error() => Err(HttpError::ServerError),
            s if s.is_success() => Ok(()),
            _ => Err(HttpError::Other(status.to_string())),
        }
    }
    pub async fn post<T: DeserializeOwned>(&self, path: &str, body: &Value) -> Result<String, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header("x-bot-token", &self.token)
            .body(body.to_string())
            .send()
            .await?;
        Self::status(response.status())?;
        Ok(response.json().await?)
    }
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("x-bot-token", &self.token)
            .send()
            .await?;
        Self::status(response.status())?;
        Ok(response.json().await?)
    }
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<String, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client
            .delete(&url)
            .header("x-bot-token", &self.token)
            .send()
            .await?
            .error_for_status()?;
        Self::status(response.status())?;
        Ok(response.json().await?)
    }
    pub async fn patch<T: DeserializeOwned>(&self, path: &str, body: &str) -> Result<String, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client
            .patch(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header("x-bot-token", &self.token)
            .body(body.to_string())
            .send()
            .await?
            .error_for_status()?;
        Self::status(response.status())?;
        Ok(response.json().await?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_get() {
        let http = Http::new("N9fHtcsWLxU4n0F42oKQ6lWLN1qQkQBxX_ZzohqpPkyAFYZ7iUap6-vrlY8EMPCi".to_string());
        let response: Result<serde_json::Value, _> = http.get("/users/@me").await;
        println!(
            "Response: {}",
            match &response {
                Ok(val) => format!("{:?}", val),
                Err(err) => format!("{:?}", err),
            }
        );
        assert!(response.is_ok());
    }
    #[tokio::test]
    async fn test_http_get_fail() {
        let http = Http::new("N9fHtcsWLxU4n0F42oKQ6lWLN1qQkQBxX_ZzohqpPkyAFYZ7iUap6-vrlY8EMPC".to_string());
        let response: Result<serde_json::Value, _> = http.get("/users/@me").await;
        println!(
            "Response: {}",
            match &response {
                Ok(val) => format!("{:?}", val),
                Err(err) => format!("{:?}", err),
            }
        );
        assert!(matches!(response.unwrap_err(), HttpError::Unauthorized));
    }
    #[tokio::test]
    async fn test_http_get_not_found() {
        let http = Http::new("N9fHtcsWLxU4n0F42oKQ6lWLN1qQkQBxX_ZzohqpPkyAFYZ7iUap6-vrlY8EMPCi".to_string());
        let response: Result<serde_json::Value, HttpError> = http.get("/nonexistent").await;
        println!(
            "Response: {}",
            match &response {
                Ok(val) => format!("{:?}", val),
                Err(err) => format!("{:?}", err),
            }
        );
        assert!(matches!(response, Err(HttpError::NotFound)));
    }
}