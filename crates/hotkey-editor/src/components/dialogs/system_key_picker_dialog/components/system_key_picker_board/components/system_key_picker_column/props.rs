use dioxus::prelude::*;

use super::components::system_key_picker_row::SystemKeyPickerRowProps;

/// One column of the board (the main keyboard, or the numpad): its already-shaped
/// rows, built by the picker hook.
#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerColumnProps {
    pub rows: Vec<SystemKeyPickerRowProps>,
}
