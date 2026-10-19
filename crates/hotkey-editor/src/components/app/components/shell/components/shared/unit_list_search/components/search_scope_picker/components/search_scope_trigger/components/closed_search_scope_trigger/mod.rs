mod model;
mod view;

pub use view::ClosedSearchScopeTriggerView;
mod style;

use crate::components::app::components::shell::components::shared::icons::ICON_CHEVRON_DOWN;
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
            span {
                dangerous_inner_html: ICON_CHEVRON_DOWN,
            }
        }
    }
}

assert_component!(ClosedSearchScopeTrigger);
