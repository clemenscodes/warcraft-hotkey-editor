use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The breadcrumbs' only input: the active category signal it reads and its tabs
/// write.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsProps {
    pub active_category: Signal<SystemHotkeysCategory>,
}
