use std::{collections::HashMap, fmt};
use serde::{Serialize, Deserialize};
use crate::context::Context;
use crate::http::HttpError;
use crate::model::ready::{Member};
use crate::model::user::User;

pub struct SendMessage {
    pub content: String,
    pub nonce: Option<String>,
    pub attachments: Vec<String>,
    pub replies: HashMap<String, bool>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub _id: String,
    pub nonce: Option<String>,
    pub channel: String,
    pub author: String,
    pub user: Option<User>,
    pub member: Option<Member>,
    pub content: String,
    pub mentions: Option<Vec<String>>,
    pub attachments: Option<Vec<MessageAttachments>>,
    pub edited: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  MessageAttachments {
    pub _id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: MessageMetadata,
    pub content_type: String,
    pub size: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageMetadata {
    #[serde(rename = "type")]
    pub _type: String,
    pub width: usize,
    pub height: usize,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.content.is_empty() {
            write!(f, "Channel: {}, Author: {}", self.channel, self.author)
        } else {
            write!(f, "Channel: {}, Author: {}, Content: {}", self.channel, self.author, self.content)
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct SendMessagePayload {
    pub content: String,
    pub nonce: Option<String>,
    pub attachments: Option<Vec<String>>,
    pub replies: Option<Vec<Replies>>,
    pub embeds: Option<Vec<Embed>>
}
#[derive(Serialize, Deserialize)]
pub struct Replies {
    pub id: String,
    pub mention: bool,
    pub fail_if_not_exists: Option<bool>,
}
#[derive(Serialize, Deserialize)]
pub struct Embed {
    pub icon_url: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub colour: Option<String>,
}
pub struct ReplyBuilder<'a> {
    pub(crate) message: &'a Message,
    pub(crate) ctx: &'a Context,
    pub(crate) content: Option<String>,
    pub(crate) nonce: Option<String>,
    pub(crate) attachments: Option<Vec<String>>,
    pub(crate) replies: Option<Vec<Replies>>,
    pub(crate) embeds: Option<Vec<Embed>>,
}
pub struct EditMessageBuilder<'a> {
    pub(crate) message: &'a Message,
    pub(crate) ctx: &'a Context,
    pub(crate) content: Option<String>,
    pub(crate) embeds: Option<Vec<Embed>>,
}
#[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct EditMessagePayload {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
}
impl Message {
    pub fn reply<'a>(&'a self, ctx: &'a Context) -> ReplyBuilder<'a> {
        ReplyBuilder {
            message: self,
            ctx,
            content: None,
            nonce: None,
            attachments: None,
            replies: Some(vec![Replies {
                id: self._id.clone(),
                mention: true,
                fail_if_not_exists: Some(true),
            }]),
            embeds: None,
        }
    }
    pub async fn delete(&self, ctx: &Context) -> Result<(), String> {
        let url = format!("/channels/{}/messages/{}", self.channel, self._id);
        let response = ctx.http.delete(&url).await.map_err(|e| HttpError::from(e));
        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete message: {}", e)),
        }
    }
    pub async fn pin(&self, ctx: &Context) -> Result<(), HttpError> {
        let url = format!("/channels/{}/messages/{}/pin", self.channel, self._id);
        let response = ctx.http.post_empty(&url).await;
        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(HttpError::from(e)),
        }
    }
    pub async fn unpin(&self, ctx: &Context) -> Result<(), HttpError> {
        let url = format!("/channels/{}/pins/{}", self.channel, self._id);
        let response = ctx.http.delete(&url).await;
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
    pub(crate) async fn send_reply(
        &self,
        ctx: &Context,
        payload: SendMessagePayload,
    ) -> Result<Message, String> {
        let url = format!("/channels/{}/messages", self.channel);

        let payload_json = serde_json::to_value(&payload)
            .map_err(|e| format!("Failed to serialize payload: {}", e))?;

        let raw_resp = ctx.http.post::<Message>(&url, &payload_json).await
            .map_err(|e| format!("HTTP request failed: {}", e));

        println!("Raw: {:?}", raw_resp);

        Ok(raw_resp?)
    }
}