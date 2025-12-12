use crate::builders::CreateEmbed;
use crate::http::{HttpClient, HttpError};
use crate::model::channel::ChannelId;
use crate::model::message::Message;
use crate::model::message::Replies;
use serde::Serialize;
use crate::http::routing::Route;

#[derive(Debug, Default, Serialize)]
pub struct CreateMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Replies>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<CreateEmbed>,
}
impl CreateMessage {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
    pub fn nonce(mut self, nonce: Option<String>) -> Self {
        self.nonce = nonce;
        self
    }
    pub fn attachments(mut self, attachments: Vec<String>) -> Self {
        self.attachments = attachments;
        self
    }
    /// Add replies using message ID's
    ///
    /// **Note**: Having more than 5 will cause API error, and you can't use the same ID twice
    pub fn replies(mut self, replies: Replies) -> Self {
        self.replies = Some(vec![replies]);
        self
    }
    /// Add multiple embeds for the message.
    ///
    /// **Note**: This will keep all existing embeds. Use [`Self::embeds()`] to replace existing
    /// embeds.
    pub fn add_embeds(mut self, embeds: Vec<CreateEmbed>) -> Self {
        self.embeds.extend(embeds);
        self
    }

    /// Set an embed for the message.
    ///
    /// **Note**: This will replace all existing embeds. Use [`Self::add_embed()`] to keep existing
    /// embeds.
    pub fn embed(self, embed: CreateEmbed) -> Self {
        self.embeds(vec![embed])
    }

    /// Set multiple embeds for the message.
    ///
    /// **Note**: This will replace all existing embeds. Use [`Self::add_embeds()`] to keep existing
    /// embeds.
    pub fn embeds(mut self, embeds: Vec<CreateEmbed>) -> Self {
        self.embeds = embeds;
        self
    }
    /// Sends the message
    pub(crate) async fn execute(self, http: &HttpClient, channel_id: &ChannelId) -> Result<Message, HttpError> {
        let route = Route::SendMessage {channel_id: &channel_id.0 };
        let response = http.request::<Self, (), Message>(route, Some(self), None).await?;
        Ok(response)
    }
}
