use std::fmt;
use serde::{Deserialize, Serialize};
use crate::builders::create_embed::SendableEmbed;
use crate::builders::fetch_messages::FetchMessagesBuilder;
use crate::context::Context;
use crate::http::HttpError;
use crate::model::message::{Message, MessageBuilder, Replies};

/// A lightweight wrapper around a Channel ID string.
/// This allows you to perform actions (send, fetch, delete) without needing
/// the full Channel object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(transparent)] // Serialize as "123", not {"0": "123"}
pub struct ChannelId(pub String);

impl ChannelId {
    /// Creates a builder to send a message to this channel.
    /// Usage: channel_id.send(&ctx).content("Hello").await;
    pub fn send<'a>(&self, ctx: &'a Context) -> PendingSend<'a> {
        PendingSend {
            ctx,
            channel_id: self.0.clone(),
            builder: MessageBuilder::default(),
        }
    }

    /// Creates a builder to fetch messages from this channel.
    pub fn fetch_messages<'a>(&self, ctx: &'a Context) -> FetchMessagesBuilder<'a> {
        FetchMessagesBuilder {
            channel_id: self.0.clone(),
            limit: None,
            before: None,
            after: None,
            sort: None,
            nearby: None,
            include_users: None,
            ctx,
        }
    }

    /// Fetch a specific message by ID from this channel.
    pub async fn fetch_message(&self, ctx: &Context, message_id: &str) -> Result<Message, HttpError> {
        let url = format!("/channels/{}/messages/{}", self.0, message_id);
        ctx.http.get::<Message, ()>(&url, None).await
    }

    /// Bulk delete messages from this channel.
    pub async fn bulk_delete(&self, ctx: &Context, messages: Vec<Message>) -> Result<(), HttpError> {
        if messages.is_empty() {
            return Ok(());
        }
        let url = format!("/channels/{}/messages/bulk", self.0);
        let ids: Vec<String> = messages.into_iter().map(|m| m.id).collect();
        let body = serde_json::json!({ "ids": ids });

        ctx.http.delete(url, Some(body)).await
            .map_err(|e| HttpError::from(e))
    }

    /// Fetch the full Channel object from the API using this ID.
    pub async fn to_channel(&self, ctx: &Context) -> Result<Channel, HttpError> {
        let url = format!("/channels/{}", self.0);
        ctx.http.get::<Channel, ()>(&url, None).await
    }
}

// Allows printing the ID directly: println!("{}", id);
impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Allow creating ChannelId from string literals: ChannelId::from("...")
impl From<&str> for ChannelId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for ChannelId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "channel_type")]
pub enum Channel {
    SavedMessages(SavedMessages),
    DirectMessage(DirectMessage),
    Group(Group),
    TextChannel(TextChannel),
    VoiceChannel(VoiceChannel),
}

impl Channel {
    /// Helper to get the raw ID string regardless of variant
    pub fn id(&self) -> &str {
        match self {
            Channel::SavedMessages(c) => &c.id,
            Channel::DirectMessage(c) => &c.id,
            Channel::Group(c) => &c.id,
            Channel::TextChannel(c) => &c.id,
            Channel::VoiceChannel(c) => &c.id,
        }
    }

    /// Returns the server ID if this channel belongs to a server
    pub fn server_id(&self) -> Option<&str> {
        match self {
            Channel::TextChannel(c) => Some(&c.server),
            Channel::VoiceChannel(c) => Some(&c.server),
            _ => None,
        }
    }

    pub fn create_message<'a>(&self, ctx: &'a Context) -> PendingSend<'a> {
        ChannelId(self.id().to_string()).send(ctx)
    }

    pub fn fetch_messages<'a>(&self, ctx: &'a Context) -> FetchMessagesBuilder<'a> {
        ChannelId(self.id().to_string()).fetch_messages(ctx)
    }

    pub async fn fetch_message(&self, ctx: &Context, message_id: &str) -> Result<Message, HttpError> {
        ChannelId(self.id().to_string()).fetch_message(ctx, message_id).await
    }

    pub async fn bulk_delete(&self, ctx: &Context, messages: Vec<Message>) -> Result<(), HttpError> {
        ChannelId(self.id().to_string()).bulk_delete(ctx, messages).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SavedMessages {
    #[serde(rename = "_id")]
    pub id: String,
    pub user: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectMessage {
    #[serde(rename = "_id")]
    pub id: String,
    pub active: bool,
    pub recipients: Vec<String>,
    pub last_message: Option<Message>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub owner: String,
    pub recipients: Vec<String>,
    //pub icon: Option<crate::model::attachment::File>,
    pub permissions: Option<u64>,
    pub nsfw: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextChannel {
    #[serde(rename = "_id")]
    pub id: String,
    pub server: String,
    pub name: String,
    pub description: Option<String>,
    //pub icon: Option<crate::model::attachment::File>,
    pub last_message_id: Option<String>,
    #[serde(default)]
    pub nsfw: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceChannel {
    #[serde(rename = "_id")]
    pub id: String,
    pub server: String,
    pub name: String,
    pub description: Option<String>,
    //pub icon: Option<crate::model::attachment::File>,
}


pub struct PendingSend<'a> {
    ctx: &'a Context,
    channel_id: String,
    builder: MessageBuilder,
}

impl<'a> PendingSend<'a> {
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.builder.content = Some(content.into());
        self
    }
    pub fn nonce(mut self, nonce: impl Into<String>) -> Self {
        self.builder.nonce = Some(nonce.into());
        self
    }
    pub fn attachments(mut self, attachments: Vec<impl Into<String>>) -> Self {
        self.builder.attachments = Some(attachments.into_iter().map(|s| s.into()).collect());
        self
    }
    pub fn add_attachment(mut self, attachment: impl Into<String>) -> Self {
        if self.builder.attachments.is_none() {
            self.builder.attachments = Some(vec![]);
        }
        if let Some(attachments) = &mut self.builder.attachments {
            attachments.push(attachment.into());
        }
        self
    }
    pub fn replies(mut self, replies: Vec<Replies>) -> Self {
        self.builder.replies = Some(replies);
        self
    }
    pub fn embeds(mut self, embeds: Vec<SendableEmbed>) -> Self {
        self.builder.embeds = Some(embeds);
        self
    }
    pub fn add_embed(mut self, embed: SendableEmbed) -> Self {
        if self.builder.embeds.is_none() {
            self.builder.embeds = Some(vec![]);
        }
        if let Some(embeds) = &mut self.builder.embeds {
            embeds.push(embed);
        }
        self
    }

    pub async fn send(self) -> Result<Message, HttpError> {
        // Internal struct for serialization
        #[derive(Serialize)]
        struct CreateMessageRequest {
            content: Option<String>,
            nonce: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            replies: Option<Vec<Replies>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            embeds: Option<Vec<SendableEmbed>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            attachments: Option<Vec<String>>,
        }

        let payload = CreateMessageRequest {
            content: self.builder.content,
            nonce: self.builder.nonce,
            replies: self.builder.replies,
            embeds: self.builder.embeds,
            attachments: self.builder.attachments,
        };

        let url = format!("/channels/{}/messages", self.channel_id);

        let message = self.ctx.http
            .post::<Message, _>(&url, &payload)
            .await?;

        Ok(message)
    }
}