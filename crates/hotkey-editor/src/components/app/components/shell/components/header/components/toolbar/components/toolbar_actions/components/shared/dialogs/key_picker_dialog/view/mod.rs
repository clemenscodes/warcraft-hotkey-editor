use super::state::KeyPickerCell;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

#[derive(Clone, PartialEq)]
pub struct KeyPickerDialogView {
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub open: bool,
    pub allow_conflict_pick: bool,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for KeyPickerDialogView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for KeyPickerDialogView {}
