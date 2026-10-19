use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapState;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SpecialOverrideKeyView {
    pub label: String,
    pub state: EditableKeycapState,
    pub title: String,
    pub on_activate: EventHandler<()>,
}

impl ddd::Layered for SpecialOverrideKeyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SpecialOverrideKeyView {}
