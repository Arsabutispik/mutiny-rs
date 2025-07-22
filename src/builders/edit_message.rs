use serde::{Serialize, Deserialize};
use crate::context::Context;
use crate::http::HttpError;
use crate::model::message::{EditMessageBuilder, Embed, Message};

#[derive(Serialize, Deserialize, Default)]
pub struct EditMessagePayload {
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
    // add other editable fields here if needed
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

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = Some(embeds);
        self
    }

    pub async fn send(self) -> Result<Message, HttpError> {
        let url = format!("/channels/{}/messages/{}", self.message.channel, self.message._id);

        let payload = EditMessagePayload {
            content: self.content,
            embeds: self.embeds,
        };

        let payload_json = serde_json::to_value(&payload)
            .map_err(|e| HttpError::Other(format!("Failed to serialize payload: {}", e)))?;

        self.ctx.http.patch::<Message>(&url, &payload_json).await
    }
}
