pub mod components;
mod props;
mod view;

pub use view::OpenHeroLevelTriggerView;
mod style;

use super::hero_level_trigger_number::HeroLevelTriggerNumber;
use super::shared::hero_level_trigger_label::HeroLevelTriggerLabel;
use components::open_hero_level_trigger_chevron::OpenHeroLevelTriggerChevron;
use dioxus::prelude::*;
use props::OpenHeroLevelTriggerProps;
use style::CLASS;
use tw_macro::assert_component;

/// The open look of the hero-level trigger: an accented gold border and glow, with a
/// flipped caret. Rendered by the dispatcher while the menu is open; owns its button
/// root and composes the shared label and number leaves plus its flipped caret.
#[component]
pub fn OpenHeroLevelTrigger(props: OpenHeroLevelTriggerProps) -> Element {
    let number = props.number;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            HeroLevelTriggerLabel {}
            HeroLevelTriggerNumber { number }
            OpenHeroLevelTriggerChevron {}
        }
    }
}

assert_component!(OpenHeroLevelTrigger);
