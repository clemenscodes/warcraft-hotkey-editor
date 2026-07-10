use super::view::SystemHotkeysListViewView;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// What the list editor needs: the category whose rows to list. Each row resolves its
/// own binding from the query and reads the editing section from the dialog state
/// context, so the view no longer builds a binding map or holds the loaded keys.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListViewProps {
    pub category: SystemHotkeysCategory,
}

impl From<&SystemHotkeysListViewView> for SystemHotkeysListViewProps {
    fn from(view: &SystemHotkeysListViewView) -> Self {
        let SystemHotkeysListViewView { category } = view.clone();
        Self { category }
    }
}

impl ddd::Props for SystemHotkeysListViewProps {
    type View = SystemHotkeysListViewView;
}
