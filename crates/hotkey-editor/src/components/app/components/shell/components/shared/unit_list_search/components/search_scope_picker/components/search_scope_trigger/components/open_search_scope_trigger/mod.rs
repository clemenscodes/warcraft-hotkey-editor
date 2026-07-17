mod model;
mod view;

pub use view::OpenSearchScopeTriggerView;
mod style;

use dioxus::prelude::*;
use model::OpenSearchScopeTriggerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn OpenSearchScopeTrigger(props: OpenSearchScopeTriggerModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_haspopup: "listbox",
            aria_expanded: true,
            onclick,
            span { {label} }
            span { "▴" }
        }
    }
}

assert_component!(OpenSearchScopeTrigger);
