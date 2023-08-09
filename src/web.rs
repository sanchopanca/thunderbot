use axum::{
    response::Html,
    routing::{delete, get, post},
    Extension, Router, Server,
};
use axum_extra::extract::Form;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};
use tera::{Context as TeraContext, Tera};

use crate::db::Db;

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

#[derive(Deserialize)]
struct NewRuleForm {
    name: String,
    patterns: Vec<String>,
    responses: Vec<String>,
}

pub async fn create_web_server() -> Result<(), hyper::Error> {
    let db = Db::new().await;
    let app = Router::new()
        .route("/", get(home))
        .route("/rules", get(rules_table))
        .route("/new-rule-form", get(new_rule_form))
        .route("/rules", post(create_new_rule))
        .route("/pattern-input", get(additional_pattern_input))
        .route("/response-input", get(additional_response_input))
        .route("/delete", delete(deltete_whatever))
        .layer(Extension(db));
    Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
}

pub async fn home() -> Html<String> {
    let mut ctx = TeraContext::new();
    ctx.insert("who", "world");

    Html(TEMPLATES.render("index.html", &ctx).unwrap())
}

async fn rules_table(Extension(db): Extension<Db>) -> Html<String> {
    let rules = db.get_rules().await;

    let mut ctx = TeraContext::new();
    ctx.insert("rules", &rules);
    Html(TEMPLATES.render("rules-table.html", &ctx).unwrap())
}

async fn new_rule_form() -> Html<String> {
    let id = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => format!("id{}", n.as_millis()),
        _ => "rust_hasnt_been_invented_lol".to_string(),
    };
    let mut ctx = TeraContext::new();
    ctx.insert("id", &id);
    Html(TEMPLATES.render("new-rule-form.html", &ctx).unwrap())
}

async fn create_new_rule(
    Extension(db): Extension<Db>,
    Form(form): Form<NewRuleForm>,
) -> Html<String> {
    let rule = db
        .create_rule(
            form.name,
            form.patterns
                .into_iter()
                .filter(|p| !p.is_empty())
                .collect(),
            form.responses
                .into_iter()
                .filter(|r| !r.is_empty())
                .collect(),
        )
        .await;
    let mut ctx = TeraContext::new();
    ctx.insert("rule", &rule);
    Html(TEMPLATES.render("rule-row.html", &ctx).unwrap())
}

async fn additional_pattern_input() -> Html<String> {
    Html(
        TEMPLATES
            .render("pattern-input.html", &TeraContext::new())
            .unwrap(),
    )
}

async fn additional_response_input() -> Html<String> {
    Html(
        TEMPLATES
            .render("response-input.html", &TeraContext::new())
            .unwrap(),
    )
}

async fn deltete_whatever() {}
