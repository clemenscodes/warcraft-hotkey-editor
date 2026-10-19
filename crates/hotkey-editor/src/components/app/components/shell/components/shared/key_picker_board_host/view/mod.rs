use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

#[derive(Clone, PartialEq)]
pub struct KeyPickerBoardHostView {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for KeyPickerBoardHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for KeyPickerBoardHostView {}
