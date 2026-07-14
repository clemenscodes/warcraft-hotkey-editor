use super::view::KeyPickerBoardHostView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerBoardHostModel {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl From<&KeyPickerBoardHostView> for KeyPickerBoardHostModel {
    fn from(view: &KeyPickerBoardHostView) -> Self {
        let KeyPickerBoardHostView {
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

impl ddd::Model for KeyPickerBoardHostModel {
    type View = KeyPickerBoardHostView;
}
