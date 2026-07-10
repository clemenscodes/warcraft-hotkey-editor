use crate::components::app::components::shell::components::shared::key_picker_board::KeyCell;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The published `View` contract mirroring [`KeyPickerColumnProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct KeyPickerColumnView {
    pub rows: Vec<Vec<KeyCell>>,
    pub on_pick: EventHandler<KeyCode>,
}

impl ddd::View for KeyPickerColumnView {}
