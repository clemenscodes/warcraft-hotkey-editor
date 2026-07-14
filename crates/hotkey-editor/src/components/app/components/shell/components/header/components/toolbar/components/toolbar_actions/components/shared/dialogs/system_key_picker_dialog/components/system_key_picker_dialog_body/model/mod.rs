use super::view::SystemKeyPickerDialogBodyView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerDialogBodyModel {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl From<&SystemKeyPickerDialogBodyView> for SystemKeyPickerDialogBodyModel {
    fn from(view: &SystemKeyPickerDialogBodyView) -> Self {
        let SystemKeyPickerDialogBodyView {
            columns,
            on_pick,
            on_close,
        } = view.clone();
        Self {
            columns,
            on_pick,
            on_close,
        }
    }
}

impl ddd::Model for SystemKeyPickerDialogBodyModel {
    type View = SystemKeyPickerDialogBodyView;
}
