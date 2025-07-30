use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use std::time::Duration;
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use serde_json::json;
use tokio::{net::TcpStream, spawn, sync::Mutex};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use tokio_tungstenite::tungstenite::Utf8Bytes;
use crate::{client::EventHandler, context::Context, model::user::User};
use crate::{model::{message::Message as ChatMessage, ready::Ready}};
use crate::model::user::Relation;

pub struct WebSocket {
    writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    reader: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    handler: Arc<Box<dyn EventHandler>>
}

impl WebSocket {
    pub async fn new(handler: Box<dyn EventHandler>) -> WebSocket {
        let (ws_stream, _) = connect_async("wss://ws.revolt.chat").await.unwrap();
        let (writer, reader) = ws_stream.split();

        WebSocket {
            writer: Arc::from(Mutex::new(writer)),
            reader: Arc::from(Mutex::new(reader)),
            handler: Arc::from(handler)
        }
    }

    pub async fn connect(&self, token: String) -> &WebSocket {

        self.writer.lock().await.send(Message::Text(Utf8Bytes::from(json!({
            "type": "Authenticate",
            "token": token
        }).to_string()))).await.unwrap();

        let handler_reader = Arc::clone(&self.reader);
        let handler_writer = Arc::clone(&self.writer);
        let arc_token = Arc::clone(&Arc::new(token.to_owned()));
        let arc_handler = Arc::clone(&self.handler);

        spawn(async move {
            WebSocket::handler(handler_reader, handler_writer, arc_token, arc_handler).await;
        }).await.unwrap();

        self
    }

    pub async fn handler(
        reader: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
        writer: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
        token: Arc<String>,
        event: Arc<Box<dyn EventHandler>>,
    ) {
        let mut bot: Option<User> = None;
        let writer_clone = writer.clone();
        let token_clone = token.clone();
        let event_clone = event.clone();

        // Spawn heartbeat
        spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;

                let ping = json!({
                "type": "Ping",
                "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
            });

                if let Err(e) = writer_clone.lock().await.send(Message::Text(ping.to_string().into())).await {
                    eprintln!("Heartbeat failed: {:?}", e);
                    break;
                }
            }
        });

        // Read loop
        loop {
            let msg_opt = reader.lock().await.next().await;

            let message = match msg_opt {
                Some(Ok(msg)) => msg,
                Some(Err(err)) => {
                    eprintln!("WebSocket error: {:?}", err);
                    break;
                }
                None => break,
            };

            if message.is_text() {
                let raw_text = message.to_text().unwrap_or("");
                let json_value: serde_json::Value = match serde_json::from_str(raw_text) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Invalid JSON: {:?}\nRaw: {}", e, raw_text);
                        continue;
                    }
                };

                if let Some(event_type) = json_value["type"].as_str() {
                    match event_type {
                        "Ready" => {
                            let ready: Ready = match serde_json::from_value(json_value.clone()) {
                                Ok(v) => v,
                                Err(e) => {
                                    eprintln!("Failed to parse Ready: {}", e);
                                    continue;
                                }
                            };
                            bot = Some(ready.users
                                           .iter()
                                           .find(|user| {
                                               user.relationship
                                                   .as_ref()
                                                   .map(|rels| {
                                                       rels.iter().any(|rel| match rel {
                                                           Relation::Object { status, .. } => status == "User",
                                                           Relation::StatusOnly(s) => s == "User",
                                                       })
                                                   })
                                                   .unwrap_or(false)
                                           })
                                           .cloned()
                                           .expect("Bot user not found"));

                            let status_msg = json!({
                            "type": "SetStatus",
                            "status": "Online"
                        });

                            if let Err(e) = writer.lock().await.send(Message::Text(status_msg.to_string().into())).await {
                                eprintln!("Failed to send SetStatus: {}", e);
                            }

                            let context = Context::new(&token, raw_text, writer.clone(), bot.clone().unwrap());
                            event.ready(context).await;
                        }

                        "Message" => {
                            if let Some(ref bot_user) = bot {
                                let context = Context::new(&token_clone, raw_text, writer.clone(), bot_user.clone());

                                match serde_json::from_value::<ChatMessage>(json_value) {
                                    Ok(msg) => {
                                        event_clone.message(context, msg).await;
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to parse Message: {}", e);
                                    }
                                }
                            }
                        }

                        _ => {
                            // Handle other event types if needed
                        }
                    }
                }
            } else if message.is_close() {
                println!("Received close frame, shutting down.");
                break;
            }
        }
    }
}