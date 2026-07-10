use super::state::KeyPickerCell;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// The published `View` contract mirroring [`KeyPickerProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct KeyPickerView {
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub open: bool,
    pub allow_conflict_pick: bool,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}

impl ddd::View for KeyPickerView {}
