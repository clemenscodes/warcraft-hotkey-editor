use super::components::key_picker_row::KeyPickerRowProps;
use dioxus::prelude::*;

/// The board's inputs: its already-shaped rows, the keydown handler for when it
/// holds focus, and the `pending_key` signal its focus-independent fallback listener
/// writes a pressed letter into. All are built by the picker hook; the board only
/// wires them and keeps itself keyboard-ready.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerBoardProps {
    pub rows: Vec<KeyPickerRowProps>,
    pub onkeydown: EventHandler<Event<KeyboardData>>,
    pub pending_key: Signal<Option<String>>,
}
