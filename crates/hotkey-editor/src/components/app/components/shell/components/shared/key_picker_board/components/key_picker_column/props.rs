use super::view::KeyPickerColumnView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyCell;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// One column of the board: its rows of domain key cells and the handler a pick fires.
/// The board threads the domain down; the column renders each row.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerColumnProps {
    pub rows: Vec<Vec<KeyCell>>,
    pub on_pick: EventHandler<KeyCode>,
}

impl From<&KeyPickerColumnView> for KeyPickerColumnProps {
    fn from(view: &KeyPickerColumnView) -> Self {
        let KeyPickerColumnView { rows, on_pick } = view.clone();
        Self { rows, on_pick }
    }
}

impl ddd::Props for KeyPickerColumnProps {
    type View = KeyPickerColumnView;
}
