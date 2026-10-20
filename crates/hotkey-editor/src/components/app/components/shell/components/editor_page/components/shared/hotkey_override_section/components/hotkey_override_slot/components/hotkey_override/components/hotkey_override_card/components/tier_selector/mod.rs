pub mod components;
mod model;
mod style;
mod view;

pub use view::TierSelectorView;

use components::tier_stepper::TierStepper;
use dioxus::prelude::*;
use model::TierSelectorModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TierSelector(props: TierSelectorModel) -> Element {
    if !props.show {
        return rsx! {};
    }
    let label = props.label;
    let on_previous = props.on_previous;
    let on_next = props.on_next;
    rsx! {
        div {
            class: CLASS,
            TierStepper {
                label,
                on_previous,
                on_next,
            }
        }
    }
}

assert_component!(TierSelector);
