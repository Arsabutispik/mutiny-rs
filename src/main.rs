use async_trait::async_trait;
use mutiny_rs::builders::create_message::CreateMessage;
use mutiny_rs::builders::CreateEmbed;
use mutiny_rs::client::EventHandler;
use mutiny_rs::context::Context;
use mutiny_rs::model::message::Message;
use mutiny_rs::model::ready::Ready;
use mutiny_rs::Client;
pub struct BotHandler;
#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, _ctx: Context, _client: Ready) {
        println!("--- Bot is READY! Logged in to Stoat ---");
    }
    async fn message(&self, ctx: Context, message: Message) {
        if let Some(content) = &message.content {
            if content == "!ping" {
                let embed = CreateEmbed::new().color("Green").title("Pong");
                let builder = CreateMessage::new()
                    .content("Pong!")
                    .embed(embed);
                message.reply(&ctx, builder).await.expect("Could not send message");
            }
        }
    }
}
#[tokio::main]
async fn main() {
    let mut client = Client::new("nYgcQxFVYFyVtux8PZkAeomX4BbKN6udfJtBqJLHxB9JgneL05UROKa1o0e83yYc".to_string());
    client.run(BotHandler).await;
}