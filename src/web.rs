use std::time::{SystemTime, UNIX_EPOCH};

use hypersynthetic::{html, HtmlFragment};
use rocket::form::Form;
use rocket::{delete, get, post, routes, Build, FromForm, Rocket, State};

use crate::components::RuleRow;
use crate::db::Db;

#[derive(FromForm)]
struct NewRuleForm {
    name: String,
    patterns: Vec<String>,
    responses: Vec<String>,
}

pub async fn create_web_server() -> Rocket<Build> {
    let db = Db::new().await;
    rocket::build()
        .mount(
            "/",
            routes![
                home,
                rules_table,
                new_rule_form,
                create_new_rule,
                additional_pattern_input,
                additional_response_input,
                deltete_whatever,
                modify_rule_form,
            ],
        )
        .manage(db)
}

#[get("/")]
fn home() -> HtmlFragment {
    html! {
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
    }
}

#[get("/rules")]
async fn rules_table(db: &State<Db>) -> HtmlFragment {
    let rules = db.get_rules().await;

    html! {
        <table>
            <caption>"Rules"</caption>
            <thead>
                <tr>
                    <th>"name"</th>
                    <th>"trigger"</th>
                    <th>"responses"</th>
                </tr>
            </thead>
            <tbody :for={rule in rules}>
                <RuleRow rule={ &rule }/>
            </tbody>
            <tbody id="add-new-rule">
                <tr>
                    <td colspan="3">
                        <button hx-get="/new-rule-form" hx-target="#add-new-rule" hx-swap="beforebegin">"Add +"</button>
                    </td>
                </tr>
            </tbody>
        </table>
    }
}

#[get("/new-rule-form")]
async fn new_rule_form() -> HtmlFragment {
    let id = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => format!("id{}", n.as_millis()),
        _ => "rust_hasnt_been_invented_lol".to_string(),
    };

    html! {
        <tbody>
            <tr id={ id }>
                <td>
                    <input name="name" placeholder="name" />
                </td>
                <td>
                    <input name="patterns" placeholder="pattern" />
                    <button hx-get="/pattern-input" hx-swap="beforebegin">"Add another trigger"</button>
                </td>
                <td>
                    <input name="responses" placeholder="response" />
                    <button hx-get="/response-input" hx-swap="beforebegin">"Add another response"</button>
                </td>
            </tr>
            <tr>
                <td colspan="3">
                    <button hx-post="/rules" hx-target="closest tbody" hx-include="#{id}">"Create"</button>
                </td>
            </tr>
        </tbody>
    }
}

#[get("/modify-rule-form?<rule_id>")]
async fn modify_rule_form(db: &State<Db>, rule_id: i64) -> HtmlFragment {
    let rule = db.get_rule(rule_id).await;

    html! {
        <tbody>
            <tr id="rule-form-{rule.id}">
                <td>
                    <input name="name" placeholder="name" value={ rule.name } />
                </td>
                <td>
                    <input :for={pattern in rule.patterns} name="patterns" placeholder="pattern" value={ pattern } />
                    <button hx-get="/pattern-input" hx-swap="beforebegin">"Add another trigger"</button>
                </td>
                <td>
                    <input :for={response in rule.responses} name="responses" placeholder="response" value={ response } />
                    <button hx-get="/response-input" hx-swap="beforebegin">"Add another response"</button>
                </td>
            </tr>
            <tr>
                <td colspan="3">
                    <button hx-post="/rules" hx-target="closest tbody" hx-include="#rule-form-{rule.id}">"Save"</button>
                </td>
            </tr>
        </tbody>
    }
}

#[post("/rules", data = "<form>")]
async fn create_new_rule(db: &State<Db>, form: Form<NewRuleForm>) -> HtmlFragment {
    let form = form.into_inner();
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
    RuleRow(&rule)
}

#[get("/pattern-input")]
fn additional_pattern_input() -> HtmlFragment {
    html! {
        <div style="display: flex;">
            <input name="patterns" placeholder="another pattern" />
            <button hx-delete="/delete" hx-target="closest div" hx-swap="delete">"❌"</button>
        </div>
    }
}

#[get("/response-input")]
fn additional_response_input() -> HtmlFragment {
    html! {
        <div style="display: flex;">
            <input name="responses" placeholder="another response" />
            <button hx-delete="/delete" hx-target="closest div" hx-swap="delete">"❌"</button>
        </div>
    }
}

#[delete("/delete")]
fn deltete_whatever() -> HtmlFragment {
    html! {}
}
