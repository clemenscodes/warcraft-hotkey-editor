pub mod components;
mod presentation;
mod style;

use components::hero_level_backdrop::HeroLevelBackdrop;
use components::hero_level_menu::HeroLevelMenu;
use components::hero_level_trigger::HeroLevelTrigger;
use dioxus::prelude::*;
use presentation::{HeroLevelPickerView, use_hero_level_picker};
use style::CLASS;
use tw_macro::assert_component;

/// The hero-level dropdown: the trigger, and — while open — the menu and its
/// dismissing backdrop. Owns its own open state, sources the selected level from
/// context, and mounts its own menu — so nothing is threaded in from the header.
#[component]
pub fn HeroLevelPicker() -> Element {
    let HeroLevelPickerView {
        is_open,
        level_number,
        toggle,
        level_picker_open,
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
                HeroLevelMenu { level_picker_open }
                HeroLevelBackdrop { onclick: dismiss }
            }
        }
    }
}

assert_component!(HeroLevelPicker);
