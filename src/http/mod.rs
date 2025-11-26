pub mod routing;

use crate::http::routing::Route;
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

    /// The "Master" request function.
    /// It handles URL generation, Authentication, Error Checking, and Parsing.
    ///
    /// * `route`: The Route enum (provides path and HTTP method).
    /// * `body`: Optional JSON body (for POST/PATCH).
    /// * `query`: Optional Query params (for GET/DELETE).
    pub async fn request<B, Q, T>(
        &self,
        route: Route<'_>,
        body: Option<B>,
        query: Option<&Q>,
    ) -> Result<T, HttpError>
    where
        B: Serialize,
        Q: Serialize,
        T: DeserializeOwned,
    {
        let base = self.base_url.trim_end_matches('/');
        let path = route.path().trim_start_matches('/').to_string();
        let url = format!("{}/{}", base, path);
        let method = route.method();

        let mut builder = self.client
            .request(method, &url)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("x-bot-token", &self.token);

        if let Some(data) = body {
            builder = builder.json(&data);
        }

        if let Some(q) = query {
            builder = builder.query(q);
        }

        let response = builder.send().await?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(Self::status_to_error(status, text));
        }

        // If the expected type T is generic unit `()`, we don't try to parse JSON.
        // This is a "hack" to handle empty responses like 204 No Content.
        // (Note: Rust doesn't easily let us check `T == ()` at runtime,
        // so we try to parse; if T is (), serde_json::from_str("null") or empty might fail
        // depending on the API, so we handle the specific case of empty body).

        let text = response.text().await.map_err(|e| HttpError::Other(format!("Failed to read body: {}", e)))?;

        if text.is_empty() {
            // Try to deserialize "nothing" -> T.
            // If T is (), this usually requires specific handling,
            // but often APIs return "{}" or "null" for empty.
            // If the API returns literally 0 bytes, serde might error unless we handle it.
            // For strict correctness with `()`, we often return Ok(serde_json::from_str("null")?)
            // But simpler:
            if std::any::type_name::<T>() == "()" {
                return Ok(serde_json::from_str("null").unwrap_or_else(|_| unsafe { std::mem::zeroed() }));
            }
        }

        serde_json::from_str::<T>(&text).map_err(|e| {
            HttpError::Other(format!("Failed to parse JSON: {}. Body: {}", e, text))
        })
    }
    // Helper for simple GETs
    pub async fn get<T: DeserializeOwned>(&self, route: Route<'_>) -> Result<T, HttpError> {
        self.request::<(), (), T>(route, None, None).await
    }

    // Helper for simple Body requests (POST/PATCH)
    pub async fn execute<B: Serialize, T: DeserializeOwned>(&self, route: Route<'_>, body: B) -> Result<T, HttpError> {
        self.request::<B, (), T>(route, Some(body), None).await
    }

    /// Helper to map status codes to our custom errors
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
}