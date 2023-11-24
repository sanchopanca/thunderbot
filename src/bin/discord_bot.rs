use std::env;

use thunderbot::discord::create_client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    ensure_env();
    let mut discord_client = create_client().await;
    if let Err(error) = discord_client.start().await {
        eprintln!("Discord client error: {:?}", error);
    }
}

fn ensure_env() {
    dotenv::dotenv().ok();
    let _ = env::var("DISCORD_API_TOKEN").expect("Provide DISCORD_API_TOKEN env variable");
    let _ = env::var("OPENAI_API_KEY").expect("Provide OPENAI_API_KEY env variable");
    let _ = env::var("DATABASE_URL").expect("Provide DATABASE_URL env variable");
}
