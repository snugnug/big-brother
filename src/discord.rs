use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(err) = msg.channel_id.say(&ctx.http, "Pong!").await {
                tracing::error!("Failed to sent message: {err:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("bot {} is ready", ready.user.name);
    }
}

pub async fn start() {
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(e) => return,
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client.");

    if let Err(err) = client.start().await {
        tracing::error!("Failed to start bot: {err:?}");
    }
}
