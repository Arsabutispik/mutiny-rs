use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Unauthorized (401): {0}")]
    Unauthorized(String),

    #[error("Forbidden (403): {0}")]
    Forbidden(String),

    #[error("Not Found (404): {0}")]
    NotFound(String),

    #[error("Bad Request (400): {0}")]
    BadRequest(String),

    #[error("Conflict (409): {0}")]
    Conflict(String),

    #[error("Too Many Requests (429): {0}")]
    TooManyRequests(String),

    #[error("Internal Server Error (500): {0}")]
    ServerError(String),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Unhandled status code {status}: {message}")]
    Unhandled {
        status: StatusCode,
        message: String,
    },

    #[error("Other error: {0}")]
    Other(String),
}
#[derive(Clone, Debug)]
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
    fn status_to_error(status: StatusCode, body: String) -> HttpError {
        match status {
            StatusCode::BAD_REQUEST => HttpError::BadRequest(body),
            StatusCode::UNAUTHORIZED => HttpError::Unauthorized(body),
            StatusCode::FORBIDDEN => HttpError::Forbidden(body),
            StatusCode::NOT_FOUND => HttpError::NotFound(body),
            StatusCode::CONFLICT => HttpError::Conflict(body),
            StatusCode::TOO_MANY_REQUESTS => HttpError::TooManyRequests(body),
            StatusCode::INTERNAL_SERVER_ERROR => HttpError::ServerError(body),
            _ => HttpError::Unhandled { status, message: body },
        }
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B
    ) -> Result<T, HttpError> {
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

        if !status.is_success() {
            return Err(Self::status_to_error(status, response.text().await.unwrap_or_else(|_| "No response body".to_string())));
        }

        response
            .json::<T>()
            .await
            .map_err(|e| HttpError::Other(format!("Failed to parse JSON: {}", e)))
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

        if !status.is_success() {
            return Err(Self::status_to_error(status, response.text().await.unwrap_or_else(|_| "No response body".to_string())));
        }

        Ok(())
    }

    pub async fn get<T: DeserializeOwned, Q: Serialize>(&self, path: &str, query: Option<&Q>) -> Result<T, HttpError> {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("x-bot-token", &self.token);

        if let Some(q) = query {
            request = request.query(q);
        }

        let response = request
            .send()
            .await
            .map_err(HttpError::from)?;

        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|e| HttpError::Other(format!("Failed to read body: {}", e)))?;

        if !status.is_success() {
            return Err(Self::status_to_error(status, text.clone()));
        }
        /* // Uncomment this block if you want to pretty-print the JSON response
        let pretty = serde_json::from_str::<serde_json::Value>(&text)
            .map(|v| serde_json::to_string_pretty(&v).unwrap_or(text.clone()))
            .unwrap_or(text.clone());
        println!("Pretty JSON:\n{}", pretty);*/
        let parsed = serde_json::from_str::<T>(&text)
            .map_err(|e| HttpError::Other(format!("Failed to parse JSON: {}. Body: {}", e, text)))?;

        Ok(parsed)
    }
    pub async fn delete(&self, path: String, body: Option<Value>) -> Result<(), HttpError> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self.client
            .delete(&url)
            .header("x-bot-token", &self.token);
        if let Some(b) = body {
            request = request
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .json(&b);
        }
        let response = request
            .send()
            .await
            .map_err(HttpError::from)?;
        let status = response.status();

        if !status.is_success() {
            return Err(Self::status_to_error(status, response.text().await.unwrap_or_else(|_| "No response body".to_string())));
        }

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

        if !status.is_success() {
            return Err(Self::status_to_error(status, text.clone()));
        }

        let parsed = serde_json::from_str::<T>(&text)
            .map_err(|e| HttpError::Other(format!("Failed to parse JSON: {}. Body: {}", e, text)))?;

        Ok(parsed)
    }
}
