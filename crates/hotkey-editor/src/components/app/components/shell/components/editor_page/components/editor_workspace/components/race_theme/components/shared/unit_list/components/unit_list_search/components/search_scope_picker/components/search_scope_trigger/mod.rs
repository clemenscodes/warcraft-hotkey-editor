pub mod components;
mod model;
mod view;

pub use view::SearchScopeTriggerView;

use components::closed_search_scope_trigger::ClosedSearchScopeTrigger;
use components::open_search_scope_trigger::OpenSearchScopeTrigger;
use dioxus::prelude::*;
use model::SearchScopeTriggerModel;
use tw_macro::assert_component;

#[component]
pub fn SearchScopeTrigger(props: SearchScopeTriggerModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    if props.is_open {
        rsx! {
            OpenSearchScopeTrigger {
                label,
                onclick,
            }
        }
    } else {
        rsx! {
            ClosedSearchScopeTrigger {
                label,
                onclick,
            }
        }
    }
}

assert_component!(SearchScopeTrigger);
