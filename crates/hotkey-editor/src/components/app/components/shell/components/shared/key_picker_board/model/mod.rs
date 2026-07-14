use super::cell::KeyColumn;
use super::view::KeyPickerBoardView;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerBoardModel {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl From<&KeyPickerBoardView> for KeyPickerBoardModel {
    fn from(view: &KeyPickerBoardView) -> Self {
        let KeyPickerBoardView {
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

impl ddd::Model for KeyPickerBoardModel {
    type View = KeyPickerBoardView;
}
