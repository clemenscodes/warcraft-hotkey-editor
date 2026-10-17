use super::cell::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

#[derive(Clone, PartialEq)]
pub struct KeyPickerBoardView {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for KeyPickerBoardView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for KeyPickerBoardView {}
