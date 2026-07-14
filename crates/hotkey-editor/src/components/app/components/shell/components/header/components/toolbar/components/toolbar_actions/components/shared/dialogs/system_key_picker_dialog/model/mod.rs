use super::view::SystemKeyPickerDialogView;
use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_keybinds::KeyCode;

#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerDialogModel {
    #[props(into)]
    pub title: String,
    pub current_code: KeyCode,
    pub conflicts: HashMap<KeyCode, Vec<String>>,
    pub open: bool,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl From<&SystemKeyPickerDialogView> for SystemKeyPickerDialogModel {
    fn from(view: &SystemKeyPickerDialogView) -> Self {
        let SystemKeyPickerDialogView {
            title,
            current_code,
            conflicts,
            open,
            on_pick,
            on_close,
        } = view.clone();
        Self {
            title,
            current_code,
            conflicts,
            open,
            on_pick,
            on_close,
        }
    }
}

impl ddd::Model for SystemKeyPickerDialogModel {
    type View = SystemKeyPickerDialogView;
}
