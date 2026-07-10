pub mod components;
mod props;
mod style;

use super::hero_level_trigger_number::{HeroLevelTriggerNumber, HeroLevelTriggerNumberProps};
use super::shared::hero_level_trigger_label::HeroLevelTriggerLabel;
use components::open_hero_level_trigger_chevron::OpenHeroLevelTriggerChevron;
use dioxus::prelude::*;
pub use props::OpenHeroLevelTriggerProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(OpenHeroLevelTrigger);

/// The open look of the hero-level trigger: an accented gold border and glow, with a
/// flipped caret. Rendered by the dispatcher while the menu is open; owns its button
/// root and composes the shared label and number leaves plus its flipped caret.
#[component]
pub fn OpenHeroLevelTrigger(props: OpenHeroLevelTriggerProps) -> Element {
    let number = HeroLevelTriggerNumberProps::from(&props);
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            HeroLevelTriggerLabel {}
            HeroLevelTriggerNumber { ..number }
            OpenHeroLevelTriggerChevron {}
        }
    }
}
