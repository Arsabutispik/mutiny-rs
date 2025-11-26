use serde::{Deserialize, Serialize};
use crate::model::channel::ChannelId;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum InviteKind {
    Group(Group),
    ServerInvite(ServerInvite),
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invite {
    #[serde(rename = "_id")]
    pub id: String,
    pub channel: ChannelId,
    pub creator: String,
    #[serde(flatten)]
    pub kind: InviteKind,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {

}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerInvite {
    pub server: String,
}