use axum::{
    response::Html,
    routing::{get, IntoMakeService},
    Router,
};
use hyper::{server::conn::AddrIncoming, Server};
use lazy_static::lazy_static;
use std::net::SocketAddr;
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

pub fn create_web_server() -> Server<AddrIncoming, IntoMakeService<Router>> {
    let app = Router::new().route("/", get(home));
    Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000))).serve(app.into_make_service())
}

pub async fn home() -> Html<String> {
    let mut ctx = TeraContext::new();
    ctx.insert("who", "world");

    Html(TEMPLATES.render("index.html", &ctx).unwrap())
}
