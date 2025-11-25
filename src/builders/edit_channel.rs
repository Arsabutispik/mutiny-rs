use serde::{Deserialize, Serialize};
use crate::http::{Http, HttpError};
use crate::model::channel::{Channel, ChannelId};

#[derive(Debug, Default, Serialize)]
pub struct EditChannel {
    /// Whether this channel is archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Channel description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Autumn attachment id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Channel name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this channel is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Group owner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Fields to remove from channel
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub remove: Vec<FieldsChannel>
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FieldsChannel {
    Description,
    Icon,
    DefaultPermissions,
    Voice,
}

impl EditChannel {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self
    }
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    pub fn icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }
    pub fn owner(mut self, owner: String) -> Self {
        self.owner = Some(owner);
        self
    }
    pub fn remove(mut self, remove: Vec<FieldsChannel>) -> Self {
        self.remove = remove;
        self
    }
    pub(crate) async fn execute(self, http: &Http, channel_id: &ChannelId) -> Result<Channel, HttpError> {
        let route = format!("/channels/{channel_id}/edit");
        let response = http.post(&route, &self).await?;
        Ok(response)
    }
}