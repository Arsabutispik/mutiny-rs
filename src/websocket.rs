use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use std::time::Duration;
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use serde_json::json;
use tokio::{net::TcpStream, spawn, sync::Mutex, task::JoinHandle};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use tokio_tungstenite::tungstenite::Utf8Bytes;

// Internal crate imports
use crate::{client::EventHandler, context::Context, model::user::User};
use crate::{model::{message::Message as ChatMessage, ready::Ready}};
use crate::client::ClientCache;
use crate::model::user::RelationshipStatus;

// Type aliases for readability
type WsWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WsReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub struct WebSocket {
    pub writer: Arc<Mutex<WsWriter>>,
    handler: Arc<Box<dyn EventHandler>>,
}

impl WebSocket {
    /// Connects to the Gateway, sends Auth, and spawns the background loop.
    /// Returns the Client (for sending messages) and the background Task Handle (to keep main alive).
    pub async fn connect(handler: Box<dyn EventHandler>, token: String) -> (Arc<WebSocket>, JoinHandle<()>) {

        let (ws_stream, _) = connect_async("wss://ws.revolt.chat").await.expect("Failed to connect to WebSocket");
        let (writer, reader) = ws_stream.split();

        // Wrap writer in Mutex for shared access
        let writer_arc = Arc::new(Mutex::new(writer));

        writer_arc.lock().await.send(Message::Text(Utf8Bytes::from(json!({
            "type": "Authenticate",
            "token": token
        }).to_string()))).await.expect("Failed to send Authenticate packet");

        let ws_client = Arc::new(WebSocket {
            writer: writer_arc.clone(),
            handler: Arc::from(handler),
        });

        let handler_token = token.clone();
        let handler_event = ws_client.handler.clone();

        let handle = spawn(async move {
            Self::handler(reader, writer_arc, handler_token, handler_event).await;
        });

        (ws_client, handle)
    }

    /// The main Event Loop. Running in the background.
    async fn handler(
        mut reader: WsReader,
        writer: Arc<Mutex<WsWriter>>,
        token: String,
        event: Arc<Box<dyn EventHandler>>,
    ) {
        let mut bot: Option<User> = None;

        // Since we are in a loop, this cache persists as long as the connection is alive.
        let cache = ClientCache::new();

        let hb_writer = writer.clone();
        spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;
                let ping = json!({
                    "type": "Ping",
                    "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
                });
                if hb_writer.lock().await.send(Message::Text(ping.to_string().into())).await.is_err() {
                    break; // Stop heartbeat if writer is dead
                }
            }
        });

        // --- Main Read Loop ---
        while let Some(msg_result) = reader.next().await {
            let message = match msg_result {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("WebSocket Error: {}", e);
                    break;
                }
            };

            if message.is_text() {
                let raw_text = message.to_text().unwrap_or("");

                let json_value: serde_json::Value = match serde_json::from_str(raw_text) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("JSON Parse Error: {}", e);
                        continue;
                    }
                };

                if let Some(event_type) = json_value["type"].as_str() {
                    match event_type {
                        "Ready" => {
                            match serde_json::from_value::<Ready>(json_value.clone()) {
                                Ok(ready) => {

                                    bot = ready.users.iter()
                                        .find(|u| u.relationship == Some(RelationshipStatus::User))
                                        .cloned();

                                    if bot.is_none() {
                                        eprintln!("Warning: Could not find own Bot User in Ready payload!");
                                    }
                                    cache.hydrate(&ready).await;

                                    if let Some(ref bot_user) = bot {
                                        let ctx = Context::new(
                                            &token,
                                            json_value,
                                            writer.clone(),
                                            bot_user.clone(),
                                            cache.clone()
                                        );
                                        event.ready(ctx, ready).await;
                                    }
                                }
                                Err(e) => eprintln!("Failed to parse Ready struct: {}", e),
                            }
                        }

                        "Message" => {
                            if let Some(ref bot_user) = bot {
                                let ctx = Context::new(
                                    &token,
                                    json_value.clone(),
                                    writer.clone(),
                                    bot_user.clone(),
                                    cache.clone()
                                );

                                match serde_json::from_value::<ChatMessage>(json_value) {
                                    Ok(msg) => {
                                        ctx.cache.messages.insert(msg.id.clone(), msg.clone()).await;

                                        event.message(ctx, msg).await;
                                    }
                                    Err(e) => eprintln!("Failed to parse Message: {}", e),
                                }
                            }
                        }

                        _ => {
                            // Ignore other events for now
                        }
                    }
                }
            } else if message.is_close() {
                println!("Gateway closed connection.");
                break;
            }
        }
    }
}