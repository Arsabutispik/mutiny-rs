use crate::http;
use crate::model::user::User;
use futures_util::stream::SplitSink;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

#[derive(Clone, Debug)]
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
}