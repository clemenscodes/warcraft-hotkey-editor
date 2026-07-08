pub mod components;
mod props;
mod style;

use components::hero_level_trigger_chevron::HeroLevelTriggerChevron;
use components::hero_level_trigger_label::HeroLevelTriggerLabel;
use components::hero_level_trigger_number::{HeroLevelTriggerNumber, HeroLevelTriggerNumberProps};
use dioxus::prelude::*;
pub use props::HeroLevelTriggerProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HeroLevelTrigger);

/// The dropdown trigger button: a label, the current level, and a caret. Its open
/// look is driven by the `data-open` attribute.
#[component]
pub fn HeroLevelTrigger(props: HeroLevelTriggerProps) -> Element {
    let number = HeroLevelTriggerNumberProps::from(&props);
    let is_open = props.is_open;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            "data-open": is_open,
            onclick,
            HeroLevelTriggerLabel {}
            HeroLevelTriggerNumber { ..number }
            HeroLevelTriggerChevron {}
        }
    }
}
