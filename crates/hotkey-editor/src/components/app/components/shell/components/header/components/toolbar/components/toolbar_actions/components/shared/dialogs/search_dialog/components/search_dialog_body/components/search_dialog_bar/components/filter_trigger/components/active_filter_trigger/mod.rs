mod model;
mod view;

pub use view::ActiveFilterTriggerView;
mod style;

use super::shared::filter_trigger_badge::FilterTriggerBadge;
use super::shared::filter_trigger_icon::FilterTriggerIcon;
use dioxus::prelude::*;
use model::ActiveFilterTriggerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveFilterTrigger(props: ActiveFilterTriggerModel) -> Element {
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Filter",
            aria_expanded: true,
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

assert_component!(ActiveFilterTrigger);
