use super::components::system_hotkeys_list_entry_label::SystemHotkeysListEntryLabelProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::system_hotkeys_list_view::components::system_hotkeys_list_entry::components::key_capture::KeyCaptureProps;
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// One hotkey row: the binding's display name, its section, and the shared editing
/// signal its key chip needs. The chip resolves its own binding from the query, so
/// the row threads neither loaded keys nor a binding map.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListEntryProps {
    pub section_id: WarcraftObjectId,
    pub comment: String,
    pub editing_section: Signal<Option<WarcraftObjectId>>,
}

impl From<&SystemHotkeysListEntryProps> for SystemHotkeysListEntryLabelProps {
    fn from(props: &SystemHotkeysListEntryProps) -> Self {
        let text = props.comment.clone();
        Self { text }
    }
}

impl From<&SystemHotkeysListEntryProps> for KeyCaptureProps {
    fn from(props: &SystemHotkeysListEntryProps) -> Self {
        let section_id = props.section_id;
        let editing_section = props.editing_section;
        Self {
            section_id,
            editing_section,
        }
    }
}
