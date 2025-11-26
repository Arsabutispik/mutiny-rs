use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::context::Context;
use crate::http::HttpError;
use crate::http::routing::Route;
use crate::model::message::Message;
use crate::model::ready::Member;
use crate::model::user::User;

pub struct FetchMessagesBuilder<'a> {
    pub(crate) limit: Option<usize>,
    pub(crate) before: Option<String>,
    pub(crate) after: Option<String>,
    pub(crate) sort: Option<Sort>,
    pub(crate) nearby: Option<String>,
    pub(crate) include_users: Option<bool>,
    pub(crate) channel_id: String,
    pub(crate) ctx: &'a Context,
}
#[derive(Serialize)]
struct FetchMessagesQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nearby: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_users: Option<bool>,
}
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Sort {
    Relevance,
    Latest,
    Oldest,
}
#[derive(Debug, Deserialize)]
pub struct MessageSearchResponse {
    pub messages: Vec<Message>,
    #[serde(default)]
    pub users: Vec<User>,
    #[serde(default)]
    pub members: Vec<Member>,
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MessageFetchResult {
    MessagesOnly(Vec<Message>),
    WithUsersAndMembers(MessageSearchResponse),
}
impl<'a> FetchMessagesBuilder<'a> {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }
    pub fn nearby(mut self, nearby: impl Into<String>) -> Self {
        self.nearby = Some(nearby.into());
        self
    }
    pub fn include_users(mut self, include_users: bool) -> Self {
        self.include_users = Some(include_users);
        self
    }
    pub async fn execute(&self) -> Result<Vec<Message>, HttpError> {
        let query = FetchMessagesQuery {
            limit: self.limit,
            before: self.before.as_deref(),
            after: self.after.as_deref(),
            sort: self.sort,
            nearby: self.nearby.as_deref(),
            include_users: self.include_users,
        };

        let route = Route::FetchMessages { channel_id: &self.channel_id };

        let result = self.ctx.http.request::<(), FetchMessagesQuery, MessageFetchResult>(route, None, Some(&query)).await?;

        let messages = match result {
            MessageFetchResult::MessagesOnly(list) => list,
            MessageFetchResult::WithUsersAndMembers(data) => {
                let mut messages = data.messages;

                let users_map: HashMap<String, User> = data.users.into_iter()
                    .map(|user| (user.id.clone(), user))
                    .collect();

                let members_map: HashMap<String, Member> = data.members.into_iter()
                    .map(|member| (member._id.user.clone(), member))
                    .collect();

                for message in &mut messages {
                    if let Some(user) = users_map.get(&message.author) {
                        message.user = Some(user.clone());
                    }
                    if let Some(member) = members_map.get(&message.author) {
                        message.member = Some(member.clone());
                    }
                }
                messages
            }
        };

        Ok(messages)
    }
}