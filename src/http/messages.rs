use crate::http::{Http, HttpError};
use crate::model::message::{Message, Replies};
use crate::builders::create_embed::SendableEmbed;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::model::ready::Member;
use crate::model::user::User;

#[derive(Serialize, Default, Debug, Clone)]
pub struct SendMessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Replies>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<SendableEmbed>>,
}
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Sort {
    Relevance,
    Latest,
    Oldest,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct FetchMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearby: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageSort {
    Relevance,
    Latest,
    Oldest,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct SearchMessagesBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<MessageSort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub messages: Vec<Message>,
    #[serde(default)]
    pub users: Vec<User>,
    #[serde(default)]
    pub members: Vec<Member>,
}
impl Http {
    pub async fn fetch_messages(&self, channel_id: &str, query: &FetchMessagesQuery) -> Result<Vec<Message>, HttpError> {
        self.get_with_query(&format!("/channels/{}/messages", channel_id), query).await
    }
    pub async fn send_message(&self, channel_id: &str, body: &SendMessageBody) -> Result<Message, HttpError> {
        self.post(&format!("/channels/{}/messages", channel_id), body).await
    }
    pub async fn search_messages(
        &self,
        channel_id: &str,
        body: &SearchMessagesBody
    ) -> Result<SearchResponse, HttpError> {
        let url = format!("/channels/{}/search", channel_id);
        self.post(&url, body).await
    }
    pub(crate) async fn fetch_message(&self, channel_id: &str, message_id: &str) -> Result<Message, HttpError> {
        self.get(&format!("/channels/{}/messages/{}", channel_id, message_id)).await
    }
    pub(crate) async fn delete_message(&self, channel_id: &str, message_id: &str) -> Result<(), HttpError> {
        self.delete(&format!("/channels/{}/messages/{}", channel_id, message_id)).await
    }
    pub(crate) async fn delete_messages_bulk(&self, channel_id: &str, message_ids: Vec<String>) -> Result<(), HttpError> {
        let url = format!("{}/channels/{}/messages/bulk", self.base_url, channel_id);
        let body = json!({ "ids": message_ids });

        let response = self.client.delete(&url)
            .header("x-bot-token", &self.token)
            .json(&body)
            .send()
            .await
            .map_err(HttpError::from)?;
        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(Http::status_to_error(status, text));
        }
        Ok(())
    }
}