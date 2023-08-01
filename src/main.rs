mod ai;
mod message;

use anyhow::Result;
use dashmap::DashMap;
use futures::join;
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use lazy_static::lazy_static;
use serde::Deserialize;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{ChannelId, GuildId, MessageId};
use serenity::prelude::*;
use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};
use std::{convert::Infallible, net::SocketAddr};
use tera::{Context as TeraContext, Tera};
use thiserror::Error;
use uuid::Uuid;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

lazy_static! {
    pub static ref TOKENS: DashMap<String, (u64, Instant)> = DashMap::new();
}

struct Handler;

async fn send_message(channel: ChannelId, ctx: &Context, message: &str) {
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

#[allow(dead_code)]
fn get_guild() -> GuildId {
    GuildId::from(
        env::var("DISCORD_GUILD_ID")
            .expect("Provide DISCORD_GUILD_ID env variable")
            .parse::<u64>()
            .expect("DISCORD_GUILD_ID must be integer"),
    )
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!edit") {
            send_message(
                msg.channel_id,
                &ctx,
                &format!(
                    "http://localhost:3000/?token={}",
                    generate_token(msg.author.id.0)
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

fn generate_token(user_id: u64) -> String {
    let token = Uuid::new_v4().to_string();
    TOKENS.insert(token.clone(), (user_id, Instant::now()));
    token
}

fn validate_token(token: &str) -> bool {
    match TOKENS.get(token) {
        Some(entry) => {
            let delta = Instant::now() - entry.value().1;
            delta < Duration::from_secs(900)
        }
        None => false,
    }
}

#[derive(Debug, Deserialize)]
enum RuleUpdateOperation {
    Add,
    Remove,
}

#[derive(Deserialize)]
struct RuleUpdateRequest {
    operation: RuleUpdateOperation,
    pattern: String,
    response: String,
}

async fn handle_web_request(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let params: HashMap<String, String> = request
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);
    let mut ctx = TeraContext::new();
    ctx.insert("who", "world");
    if request.method() == Method::GET {
        match params.get("token") {
            Some(token) => {
                if validate_token(token) {
                    return Ok(Response::new(
                        TEMPLATES.render("index.html", &ctx).unwrap().into(),
                    ));
                } else {
                    return Ok(Response::builder()
                        .status(403)
                        .body(TEMPLATES.render("403.html", &ctx).unwrap().into())
                        .unwrap());
                }
            }
            None => {
                return Ok(Response::builder()
                    .status(401)
                    .body(TEMPLATES.render("401.html", &ctx).unwrap().into())
                    .unwrap())
            }
        }
    }
    if request.method() == Method::POST {
        let bytes = body::to_bytes(request.into_body()).await.unwrap();
        let req: RuleUpdateRequest = serde_json::from_slice(&bytes).unwrap();

        return Ok(Response::new(
            format!("{} {} {:?}", req.pattern, req.response, req.operation).into(),
        ));
    }
    Ok(Response::new(
        TEMPLATES.render("index.html", &ctx).unwrap().into(),
    ))
}

// I'm not sure sqlite will work well in multithread env,
// so limit everything to one thread for now, even if we don't use sqlite currently
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let token = env::var("DISCORD_API_TOKEN").expect("Provide DISCORD_API_TOKEN env variable");
    let _ = env::var("OPENAI_API_KEY").expect("Provide OPENAI_API_KEY env variable");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_web_request)) });

    let server = Server::bind(&addr).serve(make_svc);

    match join!(client.start(), server) {
        (Err(client_error), Err(server_error)) => {
            eprintln!("Discord client error: {:?}", client_error);
            eprintln!("Error starting web server: {:?}", server_error);
        }
        (Err(client_error), _) => eprintln!("Discord client error: {:?}", client_error),
        (_, Err(server_error)) => eprintln!("Error starting web server: {:?}", server_error),
        _ => (),
    }
}
