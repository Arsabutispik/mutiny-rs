use serde::{Deserialize, Serialize};
use crate::context::Context;
use crate::http::HttpError;
use crate::http::routing::Route;
use crate::model::file::File;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub online: bool,
    pub discriminator: String,
    pub relationship: Option<RelationshipStatus>,
    pub username: String,
    pub avatar: Option<File>,
    pub badges: Option<u8>,
    pub bot: Option<Bot>,
    pub display_name: Option<String>,
    pub flags: Option<usize>,
    pub privileged: Option<bool>,
    pub status: Option<crate::model::ready::Status>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchProfile {
    pub background: Option<Background>,
    pub content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Background {
    pub content_type: String,
    pub filename: String,
    pub metadata: Metadata,
    pub size: usize,
    pub tag: String,
    pub _id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub height: usize,
    #[serde(rename = "type")]
    pub _type: String,
    pub width: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchUser {
    pub _id: String,
    pub username: String,
    pub avatar: Option<File>,
    pub relationship: Option<RelationshipStatus>,
    pub badges: usize,
    pub status: Option<Status>,
    pub online: bool,
    pub flags: Option<usize>,
    pub bot: Option<Bot>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bot {
    pub owner: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub text: Option<String>,
    pub presence: Option<String>,
}

/// User's relationship with another user (or themselves)
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
#[derive(Clone)]
pub enum RelationshipStatus {
    /// No relationship with other user
    #[default]
    None,
    /// Other user is us
    User,
    /// Friends with the other user
    Friend,
    /// Pending friend request to user
    Outgoing,
    /// Incoming friend request from user
    Incoming,
    /// Blocked this user
    Blocked,
    /// Blocked by this user
    BlockedOther,
}

impl User {
    pub async fn fetch_self(&self, ctx: &Context) -> Result<User, HttpError> {
        let route = Route::FetchMe;
        ctx.http.get::<User>(route).await
    }
    pub async fn fetch_user(&self, ctx: &Context, id: String) -> Result<User, HttpError> {
        let route = Route::FetchUser { user_id: &id };
        ctx.http.get::<User>(route).await
    }
    pub fn to_string(&self) -> String {
        format!("<@{}>", self.id)
    }
}