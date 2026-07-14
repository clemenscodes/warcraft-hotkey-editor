pub mod components;
mod presentation;
mod style;

use components::hero_selection_row::HeroSelectionRow;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_hotkeys_section_intro::SystemHotkeysSectionIntro;
use dioxus::prelude::*;
use presentation::{use_hero_selection_hotkeys_view, HeroSelectionHotkeysViewModel};
use style::CLASS;
use tw_macro::assert_component;

/// The hero-selection hotkey editor: the domain-supplied intro caption above three big
/// slots for selecting heroes by index. Its slots read the editing section from the
/// dialog state context, so it threads nothing.
#[component]
pub fn HeroSelectionHotkeysView() -> Element {
    let HeroSelectionHotkeysViewModel { caption } = use_hero_selection_hotkeys_view();
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysSectionIntro {
                text: caption,
            }
            HeroSelectionRow {
            


            }
        }
    }
}

assert_component!(HeroSelectionHotkeysView);
