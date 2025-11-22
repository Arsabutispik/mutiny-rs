use crate::builders::create_message::CreateMessage;
use crate::builders::fetch_messages::FetchMessagesBuilder;
use crate::context::Context;
use crate::http::HttpError;
use crate::model::message::Message;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A lightweight wrapper around a Channel ID string.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(transparent)]
pub struct ChannelId(pub String);

impl ChannelId {
    /// Creates a builder to send a message.
    pub async fn send_message(&self, ctx: &Context, builder: CreateMessage) -> Result<Message, HttpError> {
        builder.execute(&ctx.http, self).await
    }

    /// Creates a builder to fetch messages.
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
    /// Use this when you just want to check the name or type from RAM.
    /// Returns Option because it might not be cached yet.
    pub async fn get(&self, ctx: &Context) -> Option<Channel> {
        ctx.cache.channels.get(&self.0).await
    }
    /// Use this when you need fresh data or the cache returned None.
    /// Returns Result because the network might fail.
    pub async fn fetch(&self, ctx: &Context, force: Option<bool>) -> Result<Channel, HttpError> {
        if !force.unwrap_or(false) {
            if let Some(channel) = ctx.cache.channels.get(&self.0).await {
                return Ok(channel);
            }
        }
        let url = format!("/channels/{}", self.0);
        let channel = ctx.http.get::<Channel>(&url).await?;

        ctx.cache.channels.insert(self.0.clone(), channel.clone()).await;

        Ok(channel)
    }
    /// Depending on [Channel] this function does 3 things
    ///
    /// [TextChannel] tries to delete the channel,
    /// [Group] leaves or closes a group
    pub async fn delete(&self, ctx: &Context, leave_silent: Option<bool>) -> Result<(), HttpError> {
        #[derive(Serialize)]
        struct CloseQuery {
            leave_silently: bool,
        }
        let url = format!("/channels/{}", self.0);
        let query = CloseQuery {
            leave_silently: leave_silent.unwrap_or(false),
        };
        ctx.http.delete_with_query(&url, Some(&query)).await
    }
    /// Attempt to get the [Channel] using cache.
    /// Returns None if channel is not cached, use [Self::fetch()]
    /// to get a [Channel] object
    pub async fn to_channel(&self, ctx: &Context) -> Option<Channel> {
        ctx.cache.channels.get(&self.0).await
    }
}

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
    /// Get channel id
    pub fn id(&self) -> &ChannelId {
        match self {
            Channel::SavedMessages(c) => &c.id,
            Channel::DirectMessage(c) => &c.id,
            Channel::Group(c) => &c.id,
            Channel::TextChannel(c) => &c.id,
            Channel::VoiceChannel(c) => &c.id,
        }
    }
    /// Turn [Channel] to [ChannelId]
    pub fn to_id(&self) -> ChannelId {
        ChannelId(self.id().to_string())
    }
    pub fn server_id(&self) -> Option<&str> {
        match self {
            Channel::TextChannel(c) => Some(&c.server),
            Channel::Group(c) => Some(&c.name),
            Channel::VoiceChannel(c) => Some(&c.server),
            _ => None,
        }
    }
    pub fn name(&self) -> Option<&str> {
        match self {
            Channel::TextChannel(c) => Some(&c.name),
            Channel::VoiceChannel(c) => Some(&c.name),
            _ => None,
        }
    }

}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SavedMessages {
    #[serde(rename = "_id")]
    pub id: ChannelId,
    pub user: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectMessage {
    #[serde(rename = "_id")]
    pub id: ChannelId,
    pub active: bool,
    pub recipients: Vec<String>,
    pub last_message: Option<Message>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "_id")]
    pub id: ChannelId,
    pub name: String,
    pub owner: String,
    pub recipients: Vec<String>,
    pub permissions: Option<u64>,
    pub nsfw: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextChannel {
    #[serde(rename = "_id")]
    pub id: ChannelId,
    pub server: String,
    pub name: String,
    pub description: Option<String>,
    pub last_message_id: Option<String>,
    #[serde(default)]
    pub nsfw: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceChannel {
    #[serde(rename = "_id")]
    pub id: ChannelId,
    pub server: String,
    pub name: String,
    pub description: Option<String>,
}
