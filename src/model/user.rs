use crate::model::file::File;
use serde::{Deserialize, Serialize};

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
    pub status: Option<UserStatus>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotInformation {
    #[serde(rename = "owner")]
    pub owner_id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserStatus {
    pub text: Option<String>,
    pub presence: Option<Presence>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Presence {
    Online,
    Idle,
    Focus,
    Busy,
    Invisible,
}

/// User's relationship with another user (or themselves)
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
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
#[derive(Debug, Serialize)]
/// New user profile data
pub struct DataUserProfile {
    /// Text to set as user profile description
    pub content: Option<String>,
    /// Attachment ID for background
    pub background: Option<String>,
}
#[derive(Debug, Serialize)]
/// Optional fields on user object
pub enum FieldsUser {
    Avatar,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
    DisplayName,

    /// Internal field, ignore this.
    Internal,
}
#[derive(Default, Serialize)]
pub struct DataEditUser {
    /// New display name
    pub display_name: Option<String>,
    /// Attachment Id for avatar
    pub avatar: Option<String>,
    /// New user status
    pub status: Option<UserStatus>,
    /// New user profile data
    ///
    /// This is applied as a partial.
    pub profile: Option<DataUserProfile>,

    /// Bitfield of user badges
    pub badges: Option<i32>,
    /// Enum of user flags
    pub flags: Option<i32>,

    /// Fields to remove from user object
    pub remove: Option<Vec<FieldsUser>>,
}
impl DataEditUser {
    pub fn new() -> Self {
        Self::default()
    }
}
impl User {
    /// Turns the user object into a mentionable string
    pub fn to_string(&self) -> String {
        format!("<@{}>", self.id)
    }
}