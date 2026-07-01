mod hooks;
mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::slot_button::SlotButton;
use dioxus::prelude::*;
use hooks::use_control_groups_row;
pub use props::ControlGroupsRowProps;
use style::CLASS;
use warcraft_database::SystemHotkeysCategory;
assert_component!(ControlGroupsRow);

/// The ten-cell control-group strip.
#[component]
pub fn ControlGroupsRow(props: ControlGroupsRowProps) -> Element {
    let model = use_control_groups_row(&props);
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let binding_map = model.binding_map;
    let entries = SystemHotkeysCategory::ControlGroups.entries();
    rsx! {
        div { class: CLASS, style: model.frame,
            for (slot_index, entry) in entries.iter().enumerate() {
                SlotButton {
                    compact: true,
                    slot_label: format!("{}", slot_index + 1),
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
