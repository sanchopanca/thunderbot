use anyhow::Result;
use serenity::{
    async_trait,
    model::prelude::{ChannelId, GuildId, Message, MessageId, Ready},
    prelude::*,
};
use std::env;
use thiserror::Error;

use crate::{ai, auth, message};

#[allow(dead_code)]
fn get_guild() -> GuildId {
    GuildId::from(
        env::var("DISCORD_GUILD_ID")
            .expect("Provide DISCORD_GUILD_ID env variable")
            .parse::<u64>()
            .expect("DISCORD_GUILD_ID must be integer"),
    )
}

pub async fn send_message(channel: ChannelId, ctx: &Context, message: &str) {
    if let Err(why) = channel.say(&ctx.http, message).await {
        println!("Error sending message: {:?}", why);
    }
}

#[derive(Error, Debug)]
#[error("No messages found")]
struct NoMessagesError;

impl NoMessagesError {
    fn new() -> Self {
        NoMessagesError
    }
}

async fn summarize(channel: ChannelId, last_message: MessageId, ctx: &Context) -> Result<String> {
    let messages = channel
        .messages(&ctx.http, |retriever| retriever.before(last_message))
        .await?;

    if messages.is_empty() {
        eprintln!("Summarize: No messages found");
        return Err(NoMessagesError::new().into());
    }
    ai::ask_ai_for_summarization(reverse_messages(messages)).await
}

fn reverse_messages(messages: Vec<Message>) -> String {
    messages
        .into_iter()
        .rev()
        .map(|msg| format!("{}: {}\n", msg.author.name, msg.content))
        .collect()
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!edit") {
            send_message(
                msg.channel_id,
                &ctx,
                &format!(
                    "http://localhost:3000/?token={}",
                    auth::generate_token(msg.author.id.0)
                ),
            )
            .await;
        }

        if msg.content.contains("bot, what are they talking about") {
            if let Ok(message) = summarize(msg.channel_id, msg.id, &ctx).await {
                send_message(msg.channel_id, &ctx, &message).await
            }
        }

        if let Some(response) = message::respond(&msg.content) {
            send_message(msg.channel_id, &ctx, &response).await
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn create_client() -> Client {
    let token = env::var("DISCORD_API_TOKEN").expect("Provide DISCORD_API_TOKEN env variable");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client")
}
