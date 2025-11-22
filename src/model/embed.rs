use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")] // Revolt distinguishes types by the "type" field
pub enum Embed {
    /// Used for links and bot-created embeds
    Website(WebsiteEmbed),
    /// Used when a user uploads an image
    Image(ImageEmbed),
    /// Used when a user uploads a video
    Video(VideoEmbed),
    /// Sometimes used for pure text updates
    Text(WebsiteEmbed),
    /// Fallback for unknown types
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebsiteEmbed {
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub colour: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageEmbed {
    pub url: String,
    pub width: usize,
    pub height: usize,
    pub size: String, // "Large" or "Preview"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoEmbed {
    pub url: String,
    pub width: usize,
    pub height: usize,
}