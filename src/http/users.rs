use crate::http::Http;
use crate::http::HttpError;
use crate::model::user::User;

impl Http {
    /// Fetch the bot's own user
    pub async fn fetch_self(&self) -> Result<User, HttpError> {
        self.get("/users/@me").await
    }

    /// Fetch any user by ID
    pub async fn fetch_user(&self, user_id: &str) -> Result<User, HttpError> {
        self.get(&format!("/users/{}", user_id)).await
    }

    /// Example: Edit User (Requires a struct for the body)
    pub async fn edit_user<B: serde::Serialize>(&self, body: &B) -> Result<User, HttpError> {
        self.patch("/users/@me", body).await
    }
}