use dioxus::prelude::*;

use super::components::key_picker_row::KeyPickerRowProps;

/// The board's inputs: its already-shaped rows and the keydown handler that maps a
/// pressed letter to a pick. Both are built by the picker hook; the board only
/// wires them and keeps itself keyboard-focused.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerBoardProps {
    pub rows: Vec<KeyPickerRowProps>,
    pub onkeydown: EventHandler<Event<KeyboardData>>,
}
