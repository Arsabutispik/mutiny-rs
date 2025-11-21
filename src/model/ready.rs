use super::user::{Metadata, User};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::channel::Channel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ready {
    #[serde(rename = "type")]
    pub _type: String,
    pub channels: Vec<Channel>,
    pub members: Vec<Member>,
    pub servers: Vec<serde_json::Value>, // Error
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberId {
    pub server: String,
    pub user: String
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub _id: MemberId,
    pub joined_at: String,
}





#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    pub text: Option<String>,
    pub presence: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Icon {
    pub _id: String,
    pub tag: String,
    pub metadata: Metadata,
    pub content_type: String,
    pub size: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub _id: String,
    pub owner: String,
    pub name: String,
    pub description: String,
    pub channels: Vec<String>,
    pub categories: Vec<Category>,
    pub system_messages: SystemMessages,
    pub default_permissions: Option<serde_json::Value>,
    pub icon: Option<Icon>,
    pub roles: Option<Vec<HashMap<String, Role>>>,
    pub banner: Option<Icon>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    pub color: String,
    pub hoist: bool,
    pub name: String,
    pub permissions: Vec<usize>,
    pub rank: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemMessages {
    pub user_joined: Option<String>,
    pub user_left: Option<String>,
    pub user_kicked: Option<String>,
    pub user_banned: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub channels: Vec<String>,
}