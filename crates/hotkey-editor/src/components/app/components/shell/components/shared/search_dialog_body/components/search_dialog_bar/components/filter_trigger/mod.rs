pub mod components;
mod model;
mod view;

pub use view::FilterTriggerView;

use components::active_filter_trigger::ActiveFilterTrigger;
use components::idle_filter_trigger::IdleFilterTrigger;
use dioxus::prelude::*;
use model::FilterTriggerModel;
use tw_macro::assert_component;

#[component]
pub fn FilterTrigger(props: FilterTriggerModel) -> Element {
    let count = props.count;
    let onclick = props.onclick;
    match props.active {
        true => rsx! {
            ActiveFilterTrigger {
                count,
                onclick,
            }
        },
        false => rsx! {
            IdleFilterTrigger {
                count,
                onclick,
            }
        },
    }
}

assert_component!(FilterTrigger);
