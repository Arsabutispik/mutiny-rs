// src/http/routing.rs

use reqwest::Method;

pub enum Route<'a> {
    GetChannel { channel_id: &'a str },
    EditChannel { channel_id: &'a str },
    DeleteChannel { channel_id: &'a str },
    CreateInvite { channel_id: &'a str },
    AddRecipient { channel_id: &'a str, user_id: &'a str },
    RemoveRecipient { channel_id: &'a str, user_id: &'a str },
    SendMessage { channel_id: &'a str },
    EditMessage { channel_id: &'a str, message_id: &'a str },
    FetchMessages { channel_id: &'a str },
    MessagePin { channel_id: &'a str, message_id: &'a str },
    MessageUnpin { channel_id: &'a str, message_id: &'a str },
    MessageDelete { channel_id: &'a str, message_id: &'a str },
    FetchMe,
    FetchUser { user_id: &'a str },
}

impl<'a> Route<'a> {
    pub fn path(&self) -> String {
        match self {
            Route::GetChannel { channel_id } => format!("/channels/{}", channel_id),
            Route::EditChannel { channel_id } => format!("/channels/{}", channel_id),
            Route::DeleteChannel { channel_id } => format!("/channels/{}", channel_id),
            Route::CreateInvite { channel_id } => format!("/channels/{}/invites", channel_id),
            Route::AddRecipient {channel_id, user_id} => format!("/channels/{}/recipients/{}", channel_id, user_id),
            Route::RemoveRecipient {channel_id, user_id} => format!("/channels/{}/recipients/{}", channel_id, user_id),
            Route::SendMessage { channel_id } => format!("/channels/{}/messages", channel_id),
            Route::EditMessage { channel_id, message_id} => format!("/channels/{}/messages/{}", channel_id, message_id),
            Route::FetchMessages { channel_id } => format!("/channels/{}/messages", channel_id),
            Route::MessagePin {channel_id, message_id} => format!("/channels/{}/messages/{}/pin", channel_id, message_id),
            Route::MessageUnpin {channel_id, message_id} => format!("/channels/{}/messages/{}/pin", channel_id, message_id),
            Route::MessageDelete {channel_id, message_id} => format!("/channels/{}/messages/{}", channel_id, message_id),
            Route::FetchMe => "/users/@me".to_string(),
            Route::FetchUser { user_id } => format!("/users/{}", user_id),
        }
    }

    pub fn method(&self) -> Method {
        match self {
            Route::GetChannel { .. } => Method::GET,
            Route::EditChannel { .. } => Method::PATCH,
            Route::DeleteChannel { .. } => Method::DELETE,
            Route::CreateInvite { .. } => Method::POST,
            Route::AddRecipient { .. } => Method::PUT,
            Route::RemoveRecipient { .. } => Method::DELETE,
            Route::SendMessage { .. } => Method::POST,
            Route::EditMessage { .. } => Method::PATCH,
            Route::FetchMessages { .. } => Method::GET,
            Route::MessagePin { .. } => Method::POST,
            Route::MessageUnpin { .. } => Method::DELETE,
            Route::MessageDelete { .. } => Method::DELETE,
            Route::FetchMe => Method::GET,
            Route::FetchUser { .. } => Method::GET,
        }
    }
}