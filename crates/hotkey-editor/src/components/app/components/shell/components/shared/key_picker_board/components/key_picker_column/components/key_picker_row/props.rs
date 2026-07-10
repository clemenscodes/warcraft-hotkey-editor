use crate::components::app::components::shell::components::shared::key_picker_board::KeyCell;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// One row of the board: its domain key cells and the handler a pick fires. The column
/// threads the domain down; the row renders each cell as a key.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerRowProps {
    pub keys: Vec<KeyCell>,
    pub on_pick: EventHandler<KeyCode>,
}
