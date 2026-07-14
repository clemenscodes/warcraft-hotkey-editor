use super::view::KeyPickerRowView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyCell;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerRowModel {
    pub keys: Vec<KeyCell>,
    pub on_pick: EventHandler<KeyCode>,
}

impl From<&KeyPickerRowView> for KeyPickerRowModel {
    fn from(view: &KeyPickerRowView) -> Self {
        let KeyPickerRowView { keys, on_pick } = view.clone();
        Self { keys, on_pick }
    }
}

impl ddd::Model for KeyPickerRowModel {
    type View = KeyPickerRowView;
}
