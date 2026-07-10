pub mod components;
mod props;
mod view;

pub use view::ClosedHeroLevelTriggerView;
mod style;

use super::hero_level_trigger_number::HeroLevelTriggerNumber;
use super::shared::hero_level_trigger_label::HeroLevelTriggerLabel;
use components::closed_hero_level_trigger_chevron::ClosedHeroLevelTriggerChevron;
use dioxus::prelude::*;
use props::ClosedHeroLevelTriggerProps;
use style::CLASS;
use tw_macro::assert_component;

/// The resting look of the hero-level trigger: a dim gold border and no glow, with an
/// upright caret. Rendered by the dispatcher while the menu is closed; owns its button
/// root and composes the shared label and number leaves plus its upright caret.
#[component]
pub fn ClosedHeroLevelTrigger(props: ClosedHeroLevelTriggerProps) -> Element {
    let number = props.number;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            HeroLevelTriggerLabel {}
            HeroLevelTriggerNumber { number }
            ClosedHeroLevelTriggerChevron {}
        }
    }
}

assert_component!(ClosedHeroLevelTrigger);
