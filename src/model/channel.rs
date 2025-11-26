use crate::builders::create_message::CreateMessage;
use crate::builders::edit_channel::EditChannel;
use crate::builders::fetch_messages::FetchMessagesBuilder;
use crate::context::Context;
use crate::model::message::Message;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::error::Error;
use crate::http::HttpError;
use crate::http::routing::Route;
use crate::model::invite::Invite;
use crate::model::permissions::Permissions;
use crate::model::traits::{Nameable, ServerId};

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
        let route = Route::GetChannel { channel_id: &self.0 };
        let channel = ctx.http.get::<Channel>(route).await?;

        ctx.cache.channels.insert(self.0.clone(), channel.clone()).await;

        Ok(channel)
    }
    /// Depending on [Channel] this function does 3 things
    ///
    /// [TextChannel] tries to delete the channel,
    /// [Group] leaves or closes a group
    pub async fn delete(&self, ctx: &Context, leave_silent: Option<bool>) -> Result<(), HttpError> {
        #[derive(Serialize, Deserialize)]
        struct CloseQuery {
            leave_silently: bool,
        }
        let route = Route::DeleteChannel { channel_id: &self.0 };
        let query = CloseQuery {
            leave_silently: leave_silent.unwrap_or(false),
        };
        ctx.http.request::<(), CloseQuery, ()>(route, None, Some(&query)).await
    }
    pub async fn edit(&self, ctx: &Context, builder: EditChannel) -> Result<Channel, HttpError> {
        builder.execute(&ctx.http, self).await
    }
    /// Attempt to get the [Channel] using cache.
    /// Returns [None] if channel is not cached, use [Self::fetch()]
    /// to get a [Channel] object
    pub async fn to_channel(&self, ctx: &Context) -> Option<Channel> {
        ctx.cache.channels.get(&self.0).await
    }
    pub async fn create_invite(&self, ctx: &Context) -> Result<Invite, HttpError> {
        let route = Route::CreateInvite { channel_id: &self.0 };
        ctx.http.request::<(), (), Invite>(route, None, None).await
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
pub enum ChannelKind {
    SavedMessages(SavedMessages),
    DirectMessage(DirectMessage),
    Group(Group),
    TextChannel(TextChannel),
    VoiceChannel(VoiceChannel),

    #[serde(other)]
    Unknown,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    #[serde(rename = "_id")]
    pub id: ChannelId,

    #[serde(flatten)]
    pub kind: ChannelKind,
}
impl Channel {
    pub async fn send_message(&self, ctx: &Context, builder: CreateMessage) -> Result<Message, HttpError> {
        self.id.send_message(ctx, builder).await
    }
    /// Get the channel as text
    /// # Example
    /// ```rust
    /// if let Some(channel) = ctx.cache.channels.get(&message.channel.0).await {
    ///     if let Some(text_channel) = channel.as_text() {
    ///         println!("--- Text Channel: {} ---", text_channel.name);
    ///     } else {
    ///         println!("--- Not a Text Channel ---");
    ///     }
    /// }
    /// ```
    pub fn as_text(&self) -> Option<&TextChannel> {
        match &self.kind {
            ChannelKind::TextChannel(c) => Some(c), // 'c' is already a reference here
            _ => None,
        }
    }

    pub fn as_voice(&self) -> Option<&VoiceChannel> {
        match &self.kind {
            ChannelKind::VoiceChannel(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_group(&self) -> Option<&Group> {
        match &self.kind {
            ChannelKind::Group(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_dm(&self) -> Option<&DirectMessage> {
        match &self.kind {
            ChannelKind::DirectMessage(c) => Some(c),
            _ => None,
        }
    }
    pub async fn create_invite(&self, ctx: &Context) -> Result<Invite, Error> {
        // Allow TextChannel OR Group
        let is_allowed = matches!(
            self.kind,
            ChannelKind::TextChannel(_) | ChannelKind::Group(_)
        );

        if !is_allowed {
            return Err(Error::InvalidChannelType(
                "Invites can only be created for Text Channels or Groups".into()
            ));
        }

        // Delegate to the ID logic (which sends the HTTP request)
        self.id.create_invite(ctx).await.map_err(Error::from)
    }
}
impl Nameable for Channel {
    fn name(&self) -> Option<&str> {
        match &self.kind {
            ChannelKind::TextChannel(c) => Some(&c.name),
            ChannelKind::Group(c) => Some(&c.name),
            ChannelKind::VoiceChannel(c) => Some(&c.name),
            _ => None,
        }
    }
}
impl ServerId for Channel {
    fn server_id(&self) -> Option<&str> {
        match &self.kind {
            ChannelKind::TextChannel(c) => Some(&c.server),
            ChannelKind::VoiceChannel(c) => Some(&c.server),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SavedMessages {
    pub user: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectMessage {
    pub active: bool,
    pub recipients: Vec<String>,
    pub last_message: Option<Message>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    pub name: String,
    pub owner: String,
    pub recipients: Vec<String>,
    pub permissions: Option<Permissions>,
    pub nsfw: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextChannel {
    pub server: String,
    pub name: String,
    pub description: Option<String>,
    pub last_message_id: Option<String>,
    #[serde(default)]
    pub nsfw: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceChannel {
    pub server: String,
    pub name: String,
    pub description: Option<String>,
}
