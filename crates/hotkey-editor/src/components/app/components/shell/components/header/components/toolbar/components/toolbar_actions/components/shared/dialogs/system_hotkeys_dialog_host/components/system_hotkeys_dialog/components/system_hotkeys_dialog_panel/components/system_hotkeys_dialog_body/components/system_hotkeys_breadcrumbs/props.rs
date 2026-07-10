use super::view::SystemHotkeysBreadcrumbsView;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The breadcrumbs' only input: the active category signal it reads and its tabs
/// write.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsProps {
    pub active_category: Signal<SystemHotkeysCategory>,
}

impl From<&SystemHotkeysBreadcrumbsView> for SystemHotkeysBreadcrumbsProps {
    fn from(view: &SystemHotkeysBreadcrumbsView) -> Self {
        let SystemHotkeysBreadcrumbsView { active_category } = view.clone();
        Self { active_category }
    }
}

impl ddd::Props for SystemHotkeysBreadcrumbsProps {
    type View = SystemHotkeysBreadcrumbsView;
}
