mod ai;
mod auth;
mod discord;
mod message;
mod web;

use futures::join;
use std::env;

// I'm not sure sqlite will work well in multithread env,
// so limit everything to one thread for now, even if we don't use sqlite currently
#[tokio::main(flavor = "current_thread")]
async fn main() {
    ensure_env();

    let mut discord_client = discord::create_client().await;

    let server = web::create_web_server();

    match join!(discord_client.start(), server) {
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
