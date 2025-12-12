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
    pub relationship: RelationshipStatus,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub relations: Vec<Relationship>,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<File>,
    #[serde(skip_serializing_if = "crate::model::if_zero_u32", default)]
    pub badges: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<BotInformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "crate::model::if_zero_u32", default)]
    pub flags: u32,
    #[serde(skip_serializing_if = "crate::model::if_false", default)]
    pub privileged: bool,
    pub status: Option<crate::model::ready::Status>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotInformation {
    #[serde(rename = "owner")]
    pub owner_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub text: Option<String>,
    pub presence: Option<Presence>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Presence {
    Online,
    Idle,
    Focus,
    Busy,
    Invisible,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Relationship entry indicating current status with other user
pub struct Relationship {
    /// Other user's Id
    pub user_id: String,
    /// Relationship status with them
    pub status: RelationshipStatus,
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