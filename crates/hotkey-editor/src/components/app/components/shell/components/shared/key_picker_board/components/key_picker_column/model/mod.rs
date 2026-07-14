use super::view::KeyPickerColumnView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyCell;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerColumnModel {
    pub rows: Vec<Vec<KeyCell>>,
    pub on_pick: EventHandler<KeyCode>,
}

impl From<&KeyPickerColumnView> for KeyPickerColumnModel {
    fn from(view: &KeyPickerColumnView) -> Self {
        let KeyPickerColumnView { rows, on_pick } = view.clone();
        Self { rows, on_pick }
    }
}

impl ddd::Model for KeyPickerColumnModel {
    type View = KeyPickerColumnView;
}
