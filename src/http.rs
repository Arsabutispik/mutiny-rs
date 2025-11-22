use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
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
    Unhandled { status: StatusCode, message: String },
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Clone, Debug)]
pub struct Http {
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) token: String,
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

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, HttpError> {
        self.request_internal::<T, ()>(path, None).await
    }

    pub async fn get_with_query<T: DeserializeOwned, Q: Serialize>(&self, path: &str, query: &Q) -> Result<T, HttpError> {
        self.request_internal(path, Some(query)).await
    }
    async fn request_internal<T: DeserializeOwned, Q: Serialize>(&self, path: &str, query: Option<&Q>) -> Result<T, HttpError> {
        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        let url = format!("{}/{}", base, path);

        let mut request = self.client
            .get(&url)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("x-bot-token", &self.token);

        if let Some(q) = query {
            request = request.query(q);
        }

        let response = request.send().await.map_err(HttpError::from)?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(Self::status_to_error(status, text));
        }

        let text = response.text().await.map_err(|e| HttpError::Other(format!("Failed to read body: {}", e)))?;
        serde_json::from_str::<T>(&text).map_err(|e| HttpError::Other(format!("Failed to parse JSON: {}. Body: {}", e, text)))
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T, HttpError> {
        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        let url = format!("{}/{}", base, path);
        let response = self.client
            .post(&url)
            .header("x-bot-token", &self.token)
            .json(body)
            .send()
            .await
            .map_err(HttpError::from)?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(Self::status_to_error(status, text));
        }

        response.json::<T>().await.map_err(|e| HttpError::Other(format!("Parse Error: {}", e)))
    }

    pub async fn patch<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T, HttpError> {
        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        let url = format!("{}/{}", base, path);
        let response = self.client
            .patch(&url)
            .header("x-bot-token", &self.token)
            .json(body)
            .send()
            .await
            .map_err(HttpError::from)?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(Self::status_to_error(status, text));
        }

        response.json::<T>().await.map_err(|e| HttpError::Other(format!("Parse Error: {}", e)))
    }

    pub async fn delete_with_query<Q: Serialize>(&self, path: &str, query: Option<&Q>) -> Result<(), HttpError> {
        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        let url = format!("{}/{}", base, path);
        let mut request = self.client
            .delete(&url)
            .header("x-bot-token", &self.token);
        if let Some(q) = query {
            request = request.query(q);
        }
        let response = request.send().await.map_err(HttpError::from)?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();

            return Err(Self::status_to_error(status, text));
        }
        Ok(())
    }
    pub async fn delete(&self, path: &str) -> Result<(), HttpError> {
        // Pass generic unit type () as the query
        self.delete_with_query::<()>(path, None).await
    }
    /// Sends a POST request with no body, and expects no response body.
    /// Useful for actions like "Pin Message" or "Ack".
    pub async fn post_empty(&self, path: &str) -> Result<(), HttpError> {
        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        let url = format!("{}/{}", base, path);

        let response = self.client
            .post(&url)
            .header("x-bot-token", &self.token)
            // Note: No .json() body attached here
            .send()
            .await
            .map_err(HttpError::from)?;

        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(Self::status_to_error(status, text));
        }
        Ok(())
    }
}