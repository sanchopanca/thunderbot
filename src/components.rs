use hypersynthetic::{component, html, NodeCollection};
use std::fmt::Display;

#[component]
pub fn TableWihtSingleColumn<I, T>(items: I) -> NodeCollection
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    html! {
        <table>
        { items.into_iter().map(|thing| html! {
            <tr>
                <td>{ thing }</td>
            </tr>
        })}
        </table>
    }
}
