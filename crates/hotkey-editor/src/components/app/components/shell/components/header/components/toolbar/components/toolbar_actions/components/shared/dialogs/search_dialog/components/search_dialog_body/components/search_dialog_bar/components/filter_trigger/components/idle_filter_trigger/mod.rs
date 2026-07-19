mod model;
mod view;

pub use view::IdleFilterTriggerView;
mod style;

use super::shared::filter_trigger_badge::FilterTriggerBadge;
use super::shared::filter_trigger_icon::FilterTriggerIcon;
use dioxus::prelude::*;
use model::IdleFilterTriggerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IdleFilterTrigger(props: IdleFilterTriggerModel) -> Element {
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Filter",
            aria_expanded: false,
            onclick,
            FilterTriggerIcon {}
            if count > 0 {
                FilterTriggerBadge {
                    count,
                }
            }
        }
    }
}

assert_component!(IdleFilterTrigger);
