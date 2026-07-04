use super::components::system_key_picker_column::SystemKeyPickerColumnProps;
use dioxus::prelude::*;

/// The board's inputs: the main keyboard and numpad columns (already shaped) and
/// the keydown handler that maps a physical keypress to a pick. Both come from the
/// picker hook; the board only wires them and keeps itself keyboard-focused.
#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerBoardProps {
    pub columns: Vec<SystemKeyPickerColumnProps>,
    pub onkeydown: EventHandler<Event<KeyboardData>>,
}
