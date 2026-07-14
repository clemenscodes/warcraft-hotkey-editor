use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MoveHotkeyToggleView {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl ddd::View for MoveHotkeyToggleView {}
