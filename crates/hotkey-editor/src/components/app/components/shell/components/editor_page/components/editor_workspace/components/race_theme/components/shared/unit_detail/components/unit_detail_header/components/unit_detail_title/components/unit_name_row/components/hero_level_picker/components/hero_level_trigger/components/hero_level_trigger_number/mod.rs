mod model;
mod view;

pub use view::HeroLevelTriggerNumberView;
mod style;

use dioxus::prelude::*;
use model::HeroLevelTriggerNumberModel;
use style::CLASS;
use tw_macro::assert_component;

/// The current hero level, centred in the trigger.
#[component]
pub fn HeroLevelTriggerNumber(props: HeroLevelTriggerNumberModel) -> Element {
    let number = props.number;
    rsx! {
        span {
            class: CLASS,
            {number}
        }
    }
}

assert_component!(HeroLevelTriggerNumber);
