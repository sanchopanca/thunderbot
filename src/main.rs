mod ai;
mod auth;
mod discord;
mod message;

use anyhow::Result;
use futures::join;
use hyper::body;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::{convert::Infallible, net::SocketAddr};
use tera::{Context as TeraContext, Tera};

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
                if auth::validate_token(token) {
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
    ensure_env();

    let mut client = discord::create_client().await;

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

fn ensure_env() {
    let _ = env::var("DISCORD_API_TOKEN").expect("Provide DISCORD_API_TOKEN env variable");
    let _ = env::var("OPENAI_API_KEY").expect("Provide OPENAI_API_KEY env variable");
}
