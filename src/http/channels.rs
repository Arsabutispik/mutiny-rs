use crate::http::Http;
use crate::http::HttpError;
use crate::model::channel::Channel;

impl Http {
    pub async fn fetch_channel(&self, channel_id: &str) -> Result<Channel, HttpError> {
        self.get(&format!("/channels/{}", channel_id)).await
    }

    pub async fn close_channel(&self, channel_id: &str) -> Result<(), HttpError> {
        self.delete(&format!("/channels/{}", channel_id)).await
    }
}