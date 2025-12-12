use crate::client::ClientCache;
use crate::http;
use crate::model::user::User;
use futures_util::stream::SplitSink;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub struct Context {
    pub token: String,
    pub http: http::HttpClient,
    pub json: serde_json::Value,
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>,
    pub cache: ClientCache,
    pub bot: User,
}

impl Context {
    pub fn new(
        token: &str,
        json: serde_json::Value,
        writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>,
        bot: User,
        cache: ClientCache,
    ) -> Self
    {
        Self  {
            token: token.to_owned(),
            http: http::HttpClient::new(token.to_owned()),
            json,
            writer,
            cache,
            bot,
        }
    }
}