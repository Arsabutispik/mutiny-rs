use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    #[serde(rename = "_id")]
    pub id: String,
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

/// Metadata associated with a file
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(tag = "type")]
pub enum Metadata {
    /// File is just a generic uncategorized file
    #[default]
    File,
    /// File contains textual data and should be displayed as such
    Text,
    /// File is an image with specific dimensions
    Image { width: usize, height: usize },
    /// File is a video with specific dimensions
    Video { width: usize, height: usize },
    /// File is audio
    Audio,
}