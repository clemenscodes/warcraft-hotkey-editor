pub mod components;
mod model;
mod view;

pub use view::RaceScopeTriggerView;
mod style;

use components::race_scope_chevron::RaceScopeChevron;
use components::race_scope_summary::RaceScopeSummary;
use components::race_scope_swatches::RaceScopeSwatches;
use dioxus::prelude::*;
use model::RaceScopeTriggerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceScopeTrigger(props: RaceScopeTriggerModel) -> Element {
    let summary = props.summary;
    let is_open = props.is_open;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_haspopup: "true",
            aria_expanded: is_open,
            onclick,
            RaceScopeSwatches {}
            RaceScopeSummary {
                summary,
            }
            RaceScopeChevron {
                is_open,
            }
        }
    }
}

assert_component!(RaceScopeTrigger);
