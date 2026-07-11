use crate::components::app::components::shell::components::shared::key_picker_board::KeyCell;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The published `View` contract mirroring [`KeyPickerRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct KeyPickerRowView {
    pub keys: Vec<KeyCell>,
    pub on_pick: EventHandler<KeyCode>,
}

impl ddd::View for KeyPickerRowView {}
