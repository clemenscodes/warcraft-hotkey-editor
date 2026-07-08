mod props;
mod style;

use dioxus::prelude::*;
pub use props::HeroLevelTriggerNumberProps;
use style::CLASS;
use tw_macro::assert_component;
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
