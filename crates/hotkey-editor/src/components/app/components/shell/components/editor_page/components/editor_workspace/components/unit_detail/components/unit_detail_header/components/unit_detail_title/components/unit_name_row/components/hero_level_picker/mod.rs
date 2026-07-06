pub mod components;
mod logic;
mod props;
mod style;

use components::hero_level_backdrop::{HeroLevelBackdrop, HeroLevelBackdropProps};
use components::hero_level_menu::{HeroLevelMenu, HeroLevelMenuProps};
use components::hero_level_trigger::{HeroLevelTrigger, HeroLevelTriggerProps};
use dioxus::prelude::*;
pub use props::HeroLevelPickerProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HeroLevelPicker);

/// The hero-level dropdown: the trigger, and — while open — the menu and its
/// dismissing backdrop.
#[component]
pub fn HeroLevelPicker(props: HeroLevelPickerProps) -> Element {
    let trigger = HeroLevelTriggerProps::from(&props);
    let menu = HeroLevelMenuProps::from(&props);
    let backdrop = HeroLevelBackdropProps::from(&props);
    let is_open = props.is_open;
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
