use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

/// The published `View` contract mirroring [`SystemHotkeysBreadcrumbsProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsView {
    pub active_category: Signal<SystemHotkeysCategory>,
}

impl ddd::View for SystemHotkeysBreadcrumbsView {}
