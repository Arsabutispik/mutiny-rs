use crate::builders::edit_message::EditMessageBuilder;
use crate::context::Context;
use crate::http::HttpError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::model::channel::{ChannelId, PendingSend};
use crate::model::embed::Embed;
use crate::model::ready::Member;
use crate::model::user::User;

#[derive(Debug, Default)]
pub struct SendMessage {
    pub content: String,
    pub nonce: Option<String>,
    pub attachments: Vec<String>,
    pub replies: HashMap<String, bool>,
    pub embeds: Vec<Embed>,
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
    #[serde(default)]
    pub content: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Replies {
    pub id: String,
    pub mention: bool,
    pub fail_if_not_exists: Option<bool>,
}

impl Message {
    pub fn reply<'a>(&'a self, ctx: &'a Context) -> PendingSend<'a> {
        self.channel.send(ctx).replies(vec![
            Replies {
                id: self.id.clone(),
                mention: true,
                fail_if_not_exists: Some(true),
            }
        ])
    }
    pub async fn author(&self, ctx: &Context) -> Option<User> {
        ctx.cache.users.get(&self.author).await
    }

    pub async fn fetch_author(&self, ctx: &Context) -> Result<User, HttpError> {
        if let Some(user) = self.author(ctx).await {
            return Ok(user);
        }

        let user = ctx.http.fetch_user(&self.author).await?;

        ctx.cache.users.insert(user.id.clone(), user.clone()).await;

        Ok(user)
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