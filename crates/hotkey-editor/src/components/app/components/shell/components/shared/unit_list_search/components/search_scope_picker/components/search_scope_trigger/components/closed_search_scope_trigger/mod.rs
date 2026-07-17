mod model;
mod view;

pub use view::ClosedSearchScopeTriggerView;
mod style;

use dioxus::prelude::*;
use model::ClosedSearchScopeTriggerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ClosedSearchScopeTrigger(props: ClosedSearchScopeTriggerModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_haspopup: "listbox",
            aria_expanded: false,
            onclick,
            span { {label} }
            span { "▾" }
        }
    }
}

assert_component!(ClosedSearchScopeTrigger);
