use std::{collections::HashMap};
use serde::{Serialize, Deserialize};
use crate::builders::create_embed::SendableEmbed;
use crate::builders::edit_message::EditMessageBuilder;
use crate::context::Context;
use crate::http::HttpError;
use crate::model::channel::{ChannelId, PendingSend};
use crate::model::ready::{Member};
use crate::model::user::User;
pub struct SendMessage {
    pub content: String,
    pub nonce: Option<String>,
    pub attachments: Vec<String>,
    pub replies: HashMap<String, bool>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    //Rename _id to id
    #[serde(rename = "_id")]
    pub id: String,
    pub nonce: Option<String>,
    pub channel: ChannelId,
    pub author: String,
    pub user: Option<User>,
    pub member: Option<Member>,
    //Return "" if the content is null
    #[serde(default)]
    pub content: String,
    pub mentions: Option<Vec<String>>,
    pub attachments: Option<Vec<MessageAttachments>>,
    pub edited: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  MessageAttachments {
    pub _id: String,
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


#[derive(Serialize)]
pub struct SendMessagePayload {
    pub content: String,
    pub nonce: Option<String>,
    pub attachments: Option<Vec<String>>,
    pub replies: Option<Vec<Replies>>,
    pub embeds: Option<Vec<SendableEmbed>>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Replies {
    pub id: String,
    pub mention: bool,
    pub fail_if_not_exists: Option<bool>,
}
#[derive(Debug, Default)]
pub struct MessageBuilder {
    pub content: Option<String>,
    pub nonce: Option<String>,
    pub attachments: Option<Vec<String>>,
    pub replies: Option<Vec<Replies>>,
    pub embeds: Option<Vec<SendableEmbed>>,
}


#[derive(Serialize, Default)]
pub struct EditMessagePayload {
    pub content: Option<String>,
    pub embeds: Option<Vec<SendableEmbed>>,
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
    pub async fn delete(&self, ctx: &Context) -> Result<(), String> {
        let url = format!("/channels/{}/messages/{}", self.channel, self.id);
        let response = ctx.http.delete(url, None).await.map_err(|e| HttpError::from(e));
        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete message: {}", e)),
        }
    }
    pub async fn pin(&self, ctx: &Context) -> Result<(), HttpError> {
        let url = format!("/channels/{}/messages/{}/pin", self.channel, self.id);
        let response = ctx.http.post_empty(&url).await;
        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(HttpError::from(e)),
        }
    }
    pub async fn unpin(&self, ctx: &Context) -> Result<(), HttpError> {
        let url = format!("/channels/{}/pins/{}", self.channel, self.id);
        let response = ctx.http.delete(url, None).await;
        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(HttpError::from(e)),
        }
    }
    pub fn edit<'a>(&'a self, ctx: &'a Context) -> EditMessageBuilder<'a> {
        EditMessageBuilder {
            message: self,
            ctx,
            content: None,
            embeds: None,
        }
    }
    pub async fn fetch_message(&self, ctx: &Context) -> Result<Message, HttpError> {
        self.channel.fetch_message(ctx, &self.id).await
    }
}
