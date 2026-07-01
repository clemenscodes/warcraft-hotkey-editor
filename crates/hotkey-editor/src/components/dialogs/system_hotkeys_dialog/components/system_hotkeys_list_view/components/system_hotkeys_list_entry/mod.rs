pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::key_capture_cell::KeyCaptureCell;
use components::system_hotkeys_list_entry_label::SystemHotkeysListEntryLabel;
use style::CLASS;

pub use props::SystemHotkeysListEntryProps;

assert_component!(SystemHotkeysListEntry);

/// One hotkey row: the binding name beside its editable key chip.
#[component]
pub fn SystemHotkeysListEntry(props: SystemHotkeysListEntryProps) -> Element {
    let section_id = props.section_id;
    let comment = props.comment;
    let default_hotkey = props.default_hotkey;
    let default_modifier = props.default_modifier;
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    let binding_map = props.binding_map;
    rsx! {
        li {
            class: CLASS,
            SystemHotkeysListEntryLabel { text: comment }
            KeyCaptureCell {
                section_id,
                default_hotkey,
                default_modifier,
                loaded_keys,
                editing_section,
                binding_map,
            }
        }
    }
}
