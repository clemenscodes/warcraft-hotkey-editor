use super::view::KeyPickerBodyView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerBodyModel {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl From<&KeyPickerBodyView> for KeyPickerBodyModel {
    fn from(view: &KeyPickerBodyView) -> Self {
        let KeyPickerBodyView {
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

impl ddd::Model for KeyPickerBodyModel {
    type View = KeyPickerBodyView;
}
