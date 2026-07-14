use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::KeyCode;

#[derive(Clone, PartialEq)]
pub struct SystemKeyPickerDialogView {
    pub title: String,
    pub current_code: KeyCode,
    pub conflicts: HashMap<KeyCode, Vec<String>>,
    pub open: bool,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl ddd::View for SystemKeyPickerDialogView {}
