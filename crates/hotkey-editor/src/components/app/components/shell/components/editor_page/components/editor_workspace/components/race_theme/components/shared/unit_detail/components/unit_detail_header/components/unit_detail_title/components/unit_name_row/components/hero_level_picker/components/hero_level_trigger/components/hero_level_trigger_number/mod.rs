mod props;
mod view;

pub use view::HeroLevelTriggerNumberView;
mod style;

use dioxus::prelude::*;
use props::HeroLevelTriggerNumberProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(HeroLevelTriggerNumber);
