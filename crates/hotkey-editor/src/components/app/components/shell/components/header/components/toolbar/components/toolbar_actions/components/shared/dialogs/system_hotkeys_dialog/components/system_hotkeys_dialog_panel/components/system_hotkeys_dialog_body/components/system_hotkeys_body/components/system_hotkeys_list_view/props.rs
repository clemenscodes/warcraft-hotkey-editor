use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

/// What the list editor needs: the category whose rows to list and the shared
/// editing-section signal. Each row resolves its own binding from the query, so
/// the view no longer builds a binding map or holds the loaded keys.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListViewProps {
    pub category: SystemHotkeysCategory,
    pub editing_section: Signal<Option<WarcraftObjectId>>,
}
