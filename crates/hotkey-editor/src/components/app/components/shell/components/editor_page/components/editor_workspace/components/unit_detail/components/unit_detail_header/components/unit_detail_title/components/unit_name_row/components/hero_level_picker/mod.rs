pub mod components;
mod hooks;
mod style;

use components::hero_level_backdrop::HeroLevelBackdrop;
use components::hero_level_menu::HeroLevelMenu;
use components::hero_level_trigger::HeroLevelTrigger;
use dioxus::prelude::*;
use hooks::{HeroLevelPickerView, use_hero_level_picker};
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HeroLevelPicker);

/// The hero-level dropdown: the trigger, and — while open — the menu and its
/// dismissing backdrop. Owns its own open state, sources the selected level from
/// context, and mounts its own menu — so nothing is threaded in from the header.
#[component]
pub fn HeroLevelPicker() -> Element {
    let HeroLevelPickerView {
        is_open,
        trigger,
        menu,
        backdrop,
    } = use_hero_level_picker();
    rsx! {
        div {
            class: CLASS,
            HeroLevelTrigger { ..trigger }
            if is_open {
                HeroLevelMenu { ..menu }
                HeroLevelBackdrop { ..backdrop }
            }
        }
    }
}
