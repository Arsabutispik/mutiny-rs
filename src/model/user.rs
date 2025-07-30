use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: String,
    pub username: String,
    pub discriminator: String,
    pub display_name: Option<String>,
    pub avatar: Option<Avatar>,
    pub relationship: Option<RelationshipStatus>,
    pub badges: Option<u8>,
    pub status: Option<crate::model::ready::Status>,
    pub flags: Option<usize>,
    pub privileged: Option<bool>,
    pub bot: Option<Bot>,
    pub online: bool,
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
    pub avatar: Option<Avatar>,
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Avatar {
    pub _id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: Metadata,
    pub content_type: String,
    pub size: usize,
    pub deleted: Option<bool>,
    pub reported: Option<bool>,
    pub message_id: Option<String>,
    pub user_id: Option<String>,
    pub server_id: Option<String>,
    pub object_id: Option<String>,
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