mod hooks;
mod props;
mod style;

use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::slot_button::SlotButton;
use hooks::use_hero_selection_row;
use style::CLASS;

pub use props::HeroSelectionRowProps;

assert_component!(HeroSelectionRow);

/// The three-slot hero-selection row.
#[component]
pub fn HeroSelectionRow(props: HeroSelectionRowProps) -> Element {
    let model = use_hero_selection_row(&props);
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let binding_map = model.binding_map;
    let entries = SystemHotkeysCategory::HeroSelection.entries();
    rsx! {
        div {
            class: CLASS,
            style: model.frame,
            for (slot_index, entry) in entries.iter().enumerate() {
                SlotButton {
                    slot_label: format!("Hero {}", slot_index + 1),
                    section_id: entry.section_id().to_string(),
                    default_hotkey: entry.default_hotkey(),
                    default_modifier: entry.default_modifier(),
                    loaded_keys,
                    editing_section,
                    binding_map,
                }
            }
        }
    }
}
