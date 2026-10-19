use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsView {
    pub active_category: Signal<SystemHotkeysCategory>,
}

impl ddd::Layered for SystemHotkeysBreadcrumbsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysBreadcrumbsView {}
