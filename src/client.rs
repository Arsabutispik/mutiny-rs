use crate::{context::Context, model::message::Message, websocket::WebSocket};

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync + 'static {
    async fn ready(&self, _ctx: Context) {}
    async fn message(&self, _ctx: Context, _message: Message) {}
}


pub struct Client {
    pub token: String,
    pub websocket: Option<WebSocket>,
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
        // Box the trait object to satisfy the WebSocket::new requirement
        let websocket = WebSocket::new(Box::new(event_handler)).await;
        self.websocket = Some(websocket);
        self.websocket.as_mut().unwrap().connect(self.token.clone()).await;
    }
}