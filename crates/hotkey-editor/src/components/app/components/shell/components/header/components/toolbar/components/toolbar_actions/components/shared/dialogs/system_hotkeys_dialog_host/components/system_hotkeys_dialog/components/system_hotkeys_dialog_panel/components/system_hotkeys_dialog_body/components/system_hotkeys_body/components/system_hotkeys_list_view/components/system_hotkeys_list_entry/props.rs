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
