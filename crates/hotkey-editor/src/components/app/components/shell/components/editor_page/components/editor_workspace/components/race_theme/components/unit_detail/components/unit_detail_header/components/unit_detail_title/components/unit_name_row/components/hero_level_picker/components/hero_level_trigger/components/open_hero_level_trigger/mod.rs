pub mod components;
mod model;
mod view;

pub use view::OpenHeroLevelTriggerView;
mod style;

use super::hero_level_trigger_number::HeroLevelTriggerNumber;
use super::shared::hero_level_trigger_label::HeroLevelTriggerLabel;
use components::open_hero_level_trigger_chevron::OpenHeroLevelTriggerChevron;
use dioxus::prelude::*;
use model::OpenHeroLevelTriggerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn OpenHeroLevelTrigger(props: OpenHeroLevelTriggerModel) -> Element {
    let number = props.number;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            HeroLevelTriggerLabel {}
            HeroLevelTriggerNumber {
                number,
            }
            OpenHeroLevelTriggerChevron {}
        }
    }
}

assert_component!(OpenHeroLevelTrigger);
