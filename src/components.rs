use hypersynthetic::{component, html, NodeCollection};
use std::fmt::Display;

#[component]
pub fn RowsWihtSingleColumn<I, T>(items: I) -> NodeCollection
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    html! {
        { items.into_iter().map(|thing| html! {
            <tr>
                <td>{ thing }</td>
            </tr>
        })}
    }
}
