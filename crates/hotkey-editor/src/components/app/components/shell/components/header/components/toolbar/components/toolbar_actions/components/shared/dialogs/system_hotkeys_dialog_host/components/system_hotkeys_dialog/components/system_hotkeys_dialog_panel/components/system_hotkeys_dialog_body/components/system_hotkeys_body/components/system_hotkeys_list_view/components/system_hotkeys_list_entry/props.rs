use super::components::system_hotkeys_list_entry_label::SystemHotkeysListEntryLabelProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::system_hotkeys_list_view::components::system_hotkeys_list_entry::components::key_capture::KeyCaptureProps;
use dioxus::prelude::*;
use warcraft_keybinds::WarcraftObjectId;

/// One hotkey row: the binding's display name and its section. The chip resolves its
/// own binding from the query and reads the editing section from the dialog state
/// context, so the row threads neither loaded keys nor a binding map.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListEntryProps {
    pub section_id: WarcraftObjectId,
    pub comment: String,
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
        Self { section_id }
    }
}
