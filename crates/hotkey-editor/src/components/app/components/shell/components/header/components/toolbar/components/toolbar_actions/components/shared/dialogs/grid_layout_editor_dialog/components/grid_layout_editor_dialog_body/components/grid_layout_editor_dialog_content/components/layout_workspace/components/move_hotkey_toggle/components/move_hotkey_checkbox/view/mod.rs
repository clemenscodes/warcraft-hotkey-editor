use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MoveHotkeyCheckboxView {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl ddd::Layered for MoveHotkeyCheckboxView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MoveHotkeyCheckboxView {}
