use super::view::KeyPickerKeyView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyCell;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerKeyModel {
    pub cell: KeyCell,
    pub on_pick: EventHandler<KeyCode>,
}

impl From<&KeyPickerKeyView> for KeyPickerKeyModel {
    fn from(view: &KeyPickerKeyView) -> Self {
        let KeyPickerKeyView { cell, on_pick } = view.clone();
        Self { cell, on_pick }
    }
}

impl ddd::Model for KeyPickerKeyModel {
    type View = KeyPickerKeyView;
}
