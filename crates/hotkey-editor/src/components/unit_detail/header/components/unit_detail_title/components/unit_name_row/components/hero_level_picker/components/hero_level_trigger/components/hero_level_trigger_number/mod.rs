mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::HeroLevelTriggerNumberProps;
use style::CLASS;
assert_component!(HeroLevelTriggerNumber);

/// The current hero level, centred in the trigger.
#[component]
pub fn HeroLevelTriggerNumber(props: HeroLevelTriggerNumberProps) -> Element {
    let number = props.number;
    rsx! {
        span {
            class: CLASS,
            {number}
        }
    }
}
