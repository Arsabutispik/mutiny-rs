use crate::context::Context;
use crate::model::message::Message;
use crate::http::HttpError;

impl Context {
    pub async fn fetch_message(&self, channel_id: &str, message_id: &str, force: bool) -> Result<Message, HttpError> {
        if !force {
            if let Some(cached) = self.cache.messages.get(message_id).await {
                return Ok(cached);
            }
        }

        let message = self.http.fetch_message(channel_id, message_id).await?;

        self.cache.messages.insert(message.id.clone(), message.clone()).await;

        Ok(message)
    }

    pub async fn delete_message(&self, channel_id: &str, message_id: &str) -> Result<(), HttpError> {
        self.http.delete_message(channel_id, message_id).await?;

        self.cache.messages.invalidate(message_id).await;

        Ok(())
    }
    pub async fn delete_messages_bulk(&self, channel_id: &str, message_ids: Vec<String>) -> Result<(), HttpError> {
        self.http.delete_messages_bulk(channel_id, message_ids.clone()).await?;
        self.cache.remove_messages(message_ids).await;
        Ok(())
    }
}