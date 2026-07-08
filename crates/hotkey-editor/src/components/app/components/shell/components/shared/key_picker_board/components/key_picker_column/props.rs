use super::components::key_picker_row::KeyPickerRowProps;
use dioxus::prelude::*;

/// One column of the board: its already-shaped rows, built by the board's `From`.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerColumnProps {
    pub rows: Vec<KeyPickerRowProps>,
}
