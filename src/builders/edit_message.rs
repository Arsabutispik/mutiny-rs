use crate::builders::create_embed::SendableEmbed;
use crate::context::Context;
use crate::http::HttpError;
use crate::model::message::Message;
use serde::Serialize;
#[derive(Serialize, Default)]
pub struct EditMessagePayload {
    pub content: Option<String>,
    pub embeds: Option<Vec<SendableEmbed>>,
}
pub struct EditMessageBuilder<'a> {
    pub(crate) message: &'a Message,
    pub(crate) ctx: &'a Context,
    pub(crate) content: Option<String>,
    pub(crate) embeds: Option<Vec<SendableEmbed>>,
}


impl<'a> EditMessageBuilder<'a> {
    pub fn new(message: &'a Message, ctx: &'a Context) -> Self {
        Self {
            message,
            ctx,
            content: None,
            embeds: None,
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn embeds(mut self, embeds: Vec<SendableEmbed>) -> Self {
        self.embeds = Some(embeds);
        self
    }

    pub async fn send(self) -> Result<Message, HttpError> {
        let url = format!("/channels/{}/messages/{}", self.message.channel, self.message.id);

        let payload = EditMessagePayload {
            content: self.content,
            embeds: self.embeds,
        };

        let payload_json = serde_json::to_value(&payload)
            .map_err(|e| HttpError::Other(format!("Failed to serialize payload: {}", e)))?;

        self.ctx.http.patch::<Message>(&url, &payload_json).await
    }
}
