use crate::model::message::{Embed, Message, Replies, ReplyBuilder, SendMessagePayload};

impl<'a> ReplyBuilder<'a> {
    pub async fn send(self) -> Result<Message, String> {
        let send_message_payload = SendMessagePayload {
            content: self.content.unwrap_or_default(),
            nonce: self.nonce,
            attachments: self.attachments,
            replies: self.replies,
            embeds: self.embeds,
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
    pub fn add_attachment(mut self, attachment: impl Into<String>) -> Self {
        if self.attachments.is_none() {
            self.attachments = Some(vec![]);
        }
        if let Some(attachments) = &mut self.attachments {
            attachments.push(attachment.into());
        }
        self
    }
    pub fn replies(mut self, replies: Vec<Replies>) -> Self {
        self.replies = Some(replies);
        self
    }
    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = Some(embeds);
        self
    }
    pub fn add_embed(mut self, embed: Embed) -> Self {
        if self.embeds.is_none() {
            self.embeds = Some(vec![]);
        }
        if let Some(embeds) = &mut self.embeds {
            embeds.push(embed);
        }
        self
    }
}