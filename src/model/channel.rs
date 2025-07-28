use std::fmt;
use serde::{Deserialize, Serialize};
use crate::builders::fetch_messages::FetchMessagesBuilder;
use crate::context::Context;
use crate::http::HttpError;
use crate::model::message::{Embed, Message, MessageBuilder, Replies};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ChannelId(pub String);
impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub struct PendingSend<'a> {
    ctx: &'a Context,
    channel_id: ChannelId,
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
    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.builder.embeds = Some(embeds);
        self
    }
    pub fn add_embed(mut self, embed: Embed) -> Self {
        if self.builder.embeds.is_none() {
            self.builder.embeds = Some(vec![]);
        }
        if let Some(embeds) = &mut self.builder.embeds {
            embeds.push(embed);
        }
        self
    }

    pub async fn send(self) -> Result<Message, HttpError> {
        let url = format!("/channels/{}/messages", self.channel_id.0);
        let body = serde_json::json!({
            "content": self.builder.content,
            "nonce": self.builder.nonce,
            "replies": self.builder.replies,
            "embeds": self.builder.embeds,
            "attachments": self.builder.attachments,
        });

        let message = self.ctx.http
            .post::<Message, _>(&url, &body)
            .await?;
        Ok(message)
    }
}

impl ChannelId {
    pub fn create_message<'a>(&self, ctx: &'a Context) -> PendingSend<'a> {
        PendingSend {
            ctx,
            channel_id: self.clone(),
            builder: MessageBuilder::default(),
        }
    }
    pub fn fetch_messages<'a>(&self, ctx: &'a Context) -> FetchMessagesBuilder<'a> {
        FetchMessagesBuilder {
            channel_id: self.clone(),
            limit: None,
            before: None,
            after: None,
            sort: None,
            nearby: None,
            include_users: None,
            ctx,
        }
    }
}
