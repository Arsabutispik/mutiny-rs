use std::{collections::HashMap, fmt};
use serde::{Serialize, Deserialize};
use crate::context::Context;
use crate::http;
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
    pub user: User,
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
    pub replies: Option<Vec<Replies>>
}
#[derive(Serialize, Deserialize)]
pub struct Replies {
    pub id: String,
    pub mention: bool,
    pub fail_if_not_exists: Option<bool>,
}
pub struct ReplyBuilder<'a> {
    message: &'a Message,
    ctx: &'a Context,
    content: Option<String>,
    nonce: Option<String>,
    attachments: Option<Vec<String>>,
    replies: Option<Vec<Replies>>,
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
        }
    }

    pub async fn send_reply<'a>(
        &'a self,
        ctx: &'a Context,
        payload: SendMessagePayload,
    ) -> Result<SendMessagePayload, String> {
        let url = format!("/channels/{}/messages", self.channel);
        let payload_json = match serde_json::to_value(&payload) {
            Ok(val) => val,
            Err(e) => return Err(format!("Failed to serialize payload: {}", e)),
        };
        let response = http::Http::post::<Message>(&ctx.http, &url, &payload_json)
            .await
            .map_err(|e| HttpError::from(e));
        match response {
            Ok(json) => {
                let message: SendMessagePayload = serde_json::from_value(json.parse().unwrap())
                    .map_err(|e| format!("Failed to deserialize response: {}", e))?;
                Ok(message)
            }
            Err(e) => Err(format!("HTTP error: {}", e)),
        }
    }
}

impl<'a> ReplyBuilder<'a> {
    pub async fn send(self) -> Result<SendMessagePayload, String> {
        let send_message_payload = SendMessagePayload {
            content: self.content.unwrap_or_default(),
            nonce: self.nonce,
            attachments: self.attachments,
            replies: self.replies,
        };
        self.message.send_reply(self.ctx, send_message_payload).await
    }
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
    pub fn nonce(mut self, nonce: impl Into<String>) -> Self {
        self.nonce = Some(nonce.into());
        self
    }
    pub fn attachments(mut self, attachments: Vec<impl Into<String>>) -> Self {
        self.attachments = Some(attachments.into_iter().map(|s| s.into()).collect());
        self
    }
    pub fn replies(mut self, replies: Vec<Replies>) -> Self {
        self.replies = Some(replies);
        self
    }
}