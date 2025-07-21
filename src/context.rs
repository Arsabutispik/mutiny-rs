use crate::http;
use crate::model::message::Message;
use crate::model::ready::User;
use futures_util::stream::SplitSink;
use serde_json::{json, Error};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

#[derive(Clone)]
pub struct Context {
    pub token: String,
    pub http: http::Http,
    pub json: serde_json::Value,
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>,
    pub bot: User,
}

impl Context {
    pub fn new(
        token: &str,
        json: &str,
        writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>,
        bot: User,
    ) -> Context
    {
        Context  {
            token: token.to_owned(),
            http: http::Http::new(token.to_owned()),
            json: serde_json::from_str(json).unwrap(),
            writer,
            bot,
        }
    }
    pub async fn send_message(&self, message: &str, mention: Option<bool>) {
        let json: Result<Message, Error> = serde_json::from_value(self.json.clone());

        if let Ok(json) = json {
            let _ = http::Http::post::<Message>(&self.http, format!("{}/channels/{}/messages", http::BASE_URL, json.channel).as_str(),
                &json!({
                    "content": message,
                    "nonce": ulid::Ulid::new().to_string(),
                    "replies": [{
                        "id": json._id,
                        "mention": mention.unwrap_or(false),
                    }]
                })).await;
        }
    }
}