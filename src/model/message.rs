//! Model relating to Stoat channels

use crate::builders::create_embed::SendableEmbed;
use crate::builders::edit_message::EditMessageBuilder;
use crate::context::Context;
use crate::http::HttpError;
use crate::model::channel::{ChannelId};
use crate::model::ready::Member;
use crate::model::user::User;
use serde::{Deserialize, Serialize};
use crate::builders::create_message::CreateMessage;

#[derive(Serialize, Default, Debug, Clone)]
pub struct SendMessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Replies>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<SendableEmbed>>,
}
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Sort {
    Relevance,
    Latest,
    Oldest,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct FetchMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearby: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageSort {
    Relevance,
    Latest,
    Oldest,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct SearchMessagesBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<MessageSort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub messages: Vec<Message>,
    #[serde(default)]
    pub users: Vec<User>,
    #[serde(default)]
    pub members: Vec<Member>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: String,
    pub nonce: Option<String>,
    pub channel: ChannelId,
    pub author: String,
    pub user: Option<User>,
    pub member: Option<Member>,
    pub content: Option<String>,
    pub mentions: Option<Vec<String>>,
    pub attachments: Option<Vec<MessageAttachments>>,
    pub edited: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageAttachments {
    #[serde(rename = "_id")]
    pub id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: MessageMetadata,
    pub content_type: String,
    pub size: usize
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageMetadata {
    #[serde(rename = "type")]
    pub _type: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Replies {
    pub id: String,
    pub mention: bool,
    pub fail_if_not_exists: Option<bool>,
}
impl Replies {
    pub fn new(message_id: String) -> Self {
        Self {
            id: message_id,
            mention: true, // Default to true
            fail_if_not_exists: Some(false),
        }
    }
}
impl Message {
    /// Reply to the message object
    pub async fn reply(&self, ctx: &Context, builder: CreateMessage) -> Result<Message, HttpError> {
        let builder = builder.replies(Replies::new(self.id.clone()));
        self.channel.send_message(ctx, builder).await
    }
    pub async fn pin(&self, ctx: &Context) -> Result<(), HttpError> {
        let url = format!("/channels/{}/messages/{}/pin", self.channel.0, self.id);
        ctx.http.post_empty(&url).await
    }
    pub async fn unpin(&self, ctx: &Context) -> Result<(), HttpError> {
        let url = format!("/channels/{}/messages/{}/pin", self.channel.0, self.id);
        ctx.http.delete(&url).await
    }
    pub async fn delete(&self, ctx: &Context) -> Result<(), HttpError> {
        let url = format!("/channels/{}/messages/{}", self.channel.0, self.id);
        ctx.cache.messages.remove(&self.id).await;
        ctx.http.delete(&url).await
    }
    pub fn edit<'a>(&'a self, ctx: &'a Context) -> EditMessageBuilder<'a> {
        EditMessageBuilder {
            message: self,
            ctx,
            content: None,
            embeds: None,
        }
    }
}