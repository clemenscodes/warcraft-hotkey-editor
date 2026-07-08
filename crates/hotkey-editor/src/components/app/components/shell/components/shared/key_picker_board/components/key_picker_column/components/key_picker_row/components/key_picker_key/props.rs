use crate::components::app::components::shell::components::shared::key_picker_board::KeyCell;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// One pickable key: the cell it renders and the handler fired when it is picked.
/// Everything the button shows (label, width, disabled, visual state, conflict
/// tooltip) is derived from the cell in `logic.rs`. Every key on the board is a
/// [`KeyCode`], so the key carries no type parameter.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerKeyProps {
    pub cell: KeyCell,
    pub on_pick: EventHandler<KeyCode>,
}
