use hypersynthetic::prelude::*;
use std::fmt::Display;

use crate::db::Rule;

#[component]
pub fn TableWihtSingleColumn<I, T>(items: I) -> HtmlFragment
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    html! {
        <table>
            <tr :for={item in items}>
                <td>{ item }</td>
            </tr>
        </table>
    }
}

#[component]
pub fn RuleRow(rule: &Rule) -> HtmlFragment {
    html! {
        <tr id="rule{rule.id}">
            <td>
                <div style=" display: flex;">
                { rule.name }
                <button hx-get="/modify-rule-form" hx-target="closest tbody" hx-swap="outerHTML"
                hx-include="#modify-rule-{rule.id}">"✏️"</button>
                <input id="modify-rule-{rule.id}" name="rule_id" type="hidden" value={ rule.id } />
            </div>
            </td>
            <td>
                <TableWihtSingleColumn items={ &rule.patterns }/>
            </td>
            <td>
                <TableWihtSingleColumn items={ &rule.responses }/>
            </td>
        </tr>
    }
}
