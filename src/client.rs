use std::sync::Arc;
use moka::future::Cache;
use crate::{context::Context, websocket::WebSocket};
use crate::model::channel::Channel;
use crate::model::message::Message;
use crate::model::ready::Ready;
use crate::model::user::User;

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync + 'static {
    async fn ready(&self, _ctx: Context, _ready: Ready) {}
    async fn message(&self, _ctx: Context, _message: Message) {}
}


pub struct Client {
    pub token: String,
    pub websocket: Option<Arc<WebSocket>>,
}


impl Client {
    pub fn new(token: String) -> Self {
        Self {
            token,
            websocket: None,
        }
    }


    pub async fn run<S>(&mut self, event_handler: S)
    where
        S: EventHandler + Send + Sync + 'static,
    {
        let (websocket, handle) = WebSocket::connect(Box::new(event_handler), self.token.clone()).await;

        self.websocket = Some(websocket);


        // This pauses the main function until the WebSocket disconnects or crashes.
        // If the WS dies, this line finishes, and the program can exit (or restart).
        handle.await.unwrap();

        println!("The WebSocket task has stopped. Bot is shutting down.");
    }
}
#[derive(Clone)]
pub struct ClientCache {
    pub users: Cache<String, User>,
    pub channels: Cache<String, Channel>,
    pub messages: Cache<String, Message>,
}

impl ClientCache {
    pub fn new() -> Self {
        Self {
            users: Cache::builder()
                .max_capacity(10_000)
                .build(),

            channels: Cache::builder()
                .max_capacity(1_000)
                .build(),

            messages: Cache::builder()
                .max_capacity(5_000)
                .build(),
        }
    }
    pub(crate) async fn hydrate(&self, ready: &Ready) {
        for user in &ready.users {
            self.users.insert(user.id.clone(), user.clone()).await;
        }

        for channel in &ready.channels {
            self.channels.insert(channel.id().to_string(), channel.clone()).await;
        }

        for _server in &ready.servers {

        }
    }
    /// Efficiently removes a list of messages from the cache.
    pub(crate) async fn remove_messages(&self, message_ids: Vec<String>) {
        for id in message_ids {
            // Moka's invalidate is fast and thread-safe
            self.messages.invalidate(&id).await;
        }
    }
}