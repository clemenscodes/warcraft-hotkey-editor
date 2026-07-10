use dioxus::prelude::*;

/// The toggle's current value and the change handler.
#[derive(Props, Clone, PartialEq)]
pub struct MoveHotkeyToggleProps {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}
