use std::env;

use thunderbot::web::create_web_server;

#[rocket::launch]
async fn rocket() -> _ {
    ensure_env();
    create_web_server().await
}

fn ensure_env() {
    dotenv::dotenv().ok();
    let _ = env::var("DATABASE_URL").expect("Provide DATABASE_URL env variable");
}
