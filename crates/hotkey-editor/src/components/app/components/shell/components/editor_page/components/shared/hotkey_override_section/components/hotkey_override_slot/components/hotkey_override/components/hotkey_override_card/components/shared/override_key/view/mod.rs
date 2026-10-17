use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct OverrideKeyView {
    pub label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub title: String,
    pub on_activate: EventHandler<()>,
}

impl ddd::Layered for OverrideKeyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for OverrideKeyView {}
