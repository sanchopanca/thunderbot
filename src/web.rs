use axum::{
    extract::Query,
    response::Html,
    routing::{delete, get, post},
    Extension, Router, Server,
};
use axum_extra::extract::Form;
use hypersynthetic::html;
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

#[derive(Deserialize)]
struct RuleId {
    rule_id: i64,
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
        .route("/modify-rule-form", get(modify_rule_form))
        .layer(Extension(db));
    Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service())
        .await
}

pub async fn home() -> Html<String> {
    Html(html! {
        <!DOCTYPE html>
        <html lang="en">

            <head>
                <title>{ "Slackbot" }</title>
                <meta charset="utf-8" />
                <script src="https://unpkg.com/htmx.org@1.9.4"
                    integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV"
                    crossorigin="anonymous"></script>
                <link rel="stylesheet" href="https://unpkg.com/missing.css@1.0.9/dist/missing.min.css" />
            </head>

            <body>
                <div hx-get="/rules" hx-trigger="load"></div>
            </body>

        </html>
    }.to_html())
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

async fn modify_rule_form(Extension(db): Extension<Db>, Query(q): Query<RuleId>) -> Html<String> {
    let rule = db.get_rule(q.rule_id).await;
    let mut ctx = TeraContext::new();
    ctx.insert("rule", &rule);
    Html(TEMPLATES.render("modify-rule-form.html", &ctx).unwrap())
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
