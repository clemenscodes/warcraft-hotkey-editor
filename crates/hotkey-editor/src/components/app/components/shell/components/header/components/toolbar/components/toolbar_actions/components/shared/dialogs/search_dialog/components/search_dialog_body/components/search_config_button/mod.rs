mod model;
mod style;
mod view;

pub use view::SearchConfigButtonView;

use dioxus::prelude::*;
use model::SearchConfigButtonModel;
use style::CLASS;
use tw_macro::assert_component;

/// The disclosure trigger for the search filters. Its label carries the count of
/// active filters, so a folded panel still says how many narrow the search, and a
/// tap toggles the panel open.
#[component]
pub fn SearchConfigButton(props: SearchConfigButtonModel) -> Element {
    let label = props.label;
    let open = props.open;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_expanded: open,
            onclick,
            span { {label} }
            span { "▾" }
        }
    }
}

assert_component!(SearchConfigButton);
