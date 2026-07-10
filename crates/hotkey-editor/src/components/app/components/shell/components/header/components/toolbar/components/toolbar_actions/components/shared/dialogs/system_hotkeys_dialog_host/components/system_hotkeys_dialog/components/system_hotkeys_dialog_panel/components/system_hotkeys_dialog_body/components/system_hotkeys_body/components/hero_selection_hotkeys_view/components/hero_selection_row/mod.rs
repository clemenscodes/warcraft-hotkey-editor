mod hooks;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::slot_button::SlotButton;
use dioxus::prelude::*;
use hooks::use_hero_selection_row;
use style::CLASS;
use tw_macro::assert_component;

/// The three-slot hero-selection row.
#[component]
pub fn HeroSelectionRow() -> Element {
    let model = use_hero_selection_row();
    rsx! {
        div {
            class: CLASS,
            style: model.frame,
            for entry in model.slots {
                SlotButton {
                    slot_label: entry.slot_label.clone(),
                    section_id: entry.section_id,
                }
            }
        }
    }
}

assert_component!(HeroSelectionRow);
