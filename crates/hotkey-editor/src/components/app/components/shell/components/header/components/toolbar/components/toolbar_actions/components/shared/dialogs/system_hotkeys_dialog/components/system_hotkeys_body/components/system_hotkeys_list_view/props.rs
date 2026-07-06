use dioxus::prelude::*;
use warcraft_database::SystemHotkeysCategory;
use warcraft_keybinds::{CustomKeys, WarcraftObjectId};

/// What the list editor needs: the category whose rows to list, the loaded keys it
/// edits, and the shared editing-section signal.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListViewProps {
    pub category: SystemHotkeysCategory,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub editing_section: Signal<Option<WarcraftObjectId>>,
}
