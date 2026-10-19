pub mod components;
mod presentation;
mod style;

use components::hero_level_backdrop::HeroLevelBackdrop;
use components::hero_level_menu::HeroLevelMenu;
use components::hero_level_trigger::HeroLevelTrigger;
use dioxus::prelude::*;
use presentation::{HeroLevelPickerPresentation, use_hero_level_picker};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HeroLevelPicker() -> Element {
    let HeroLevelPickerPresentation {
        is_open,
        level_number,
        toggle,
        on_close,
        dismiss,
    } = use_hero_level_picker();
    rsx! {
        div {
            class: CLASS,
            HeroLevelTrigger {
                number: level_number,
                is_open,
                onclick: toggle,
            }
            if is_open {
                HeroLevelMenu {
                    on_close,
                }
                HeroLevelBackdrop {
                    onclick: dismiss,
                }
            }
        }
    }
}

assert_component!(HeroLevelPicker);
