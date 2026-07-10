use dioxus::prelude::*;

/// The checkbox's current value and the change handler forwarded from the toggle.
#[derive(Props, Clone, PartialEq)]
pub struct MoveHotkeyCheckboxProps {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}
