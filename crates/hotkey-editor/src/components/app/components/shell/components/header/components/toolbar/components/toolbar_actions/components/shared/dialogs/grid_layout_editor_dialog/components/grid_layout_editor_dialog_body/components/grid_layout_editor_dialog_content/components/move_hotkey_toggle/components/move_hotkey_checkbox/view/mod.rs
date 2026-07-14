use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MoveHotkeyCheckboxView {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl ddd::View for MoveHotkeyCheckboxView {}
