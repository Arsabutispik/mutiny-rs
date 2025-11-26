use crate::builders::create_embed::SendableEmbed;
use crate::context::Context;
use crate::http::HttpError;
use serde::Serialize;
use crate::http::routing::Route;
use crate::model::message::Message;

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

    pub async fn edit(self, body: EditMessagePayload) -> Result<Message, HttpError> {
        let route = Route::EditMessage { channel_id: &self.message.channel.0, message_id: &self.message.id };
        let message = self.ctx.http.execute::<EditMessagePayload, Message>(route, body).await?;
        self.ctx.cache.messages.insert(message.id.clone(), message.clone()).await;
        Ok(message)
    }
}
