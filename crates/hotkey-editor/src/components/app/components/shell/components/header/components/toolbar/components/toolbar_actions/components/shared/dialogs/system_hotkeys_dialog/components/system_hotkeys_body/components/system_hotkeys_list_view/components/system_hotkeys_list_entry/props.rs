use super::components::system_hotkeys_list_entry_label::SystemHotkeysListEntryLabelProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::system_hotkeys_list_view::components::system_hotkeys_list_entry::components::key_capture_cell::KeyCaptureCellProps;
use dioxus::prelude::*;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeys, SystemBindingMap};

/// One hotkey row: the binding's display name, its section and defaults, and the
/// shared editing signal and binding map its key chip needs.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListEntryProps {
    pub section_id: String,
    pub comment: String,
    pub default_hotkey: u32,
    pub default_modifier: SystemKeybindModifier,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<String>>,
    pub binding_map: ReadSignal<SystemBindingMap>,
}

impl From<&SystemHotkeysListEntryProps> for SystemHotkeysListEntryLabelProps {
    fn from(props: &SystemHotkeysListEntryProps) -> Self {
        let text = props.comment.clone();
        Self { text }
    }
}

impl From<&SystemHotkeysListEntryProps> for KeyCaptureCellProps {
    fn from(props: &SystemHotkeysListEntryProps) -> Self {
        let section_id = props.section_id.clone();
        let default_hotkey = props.default_hotkey;
        let default_modifier = props.default_modifier;
        let loaded_keys = props.loaded_keys;
        let editing_section = props.editing_section;
        let binding_map = props.binding_map;
        Self {
            section_id,
            default_hotkey,
            default_modifier,
            loaded_keys,
            editing_section,
            binding_map,
        }
    }
}
