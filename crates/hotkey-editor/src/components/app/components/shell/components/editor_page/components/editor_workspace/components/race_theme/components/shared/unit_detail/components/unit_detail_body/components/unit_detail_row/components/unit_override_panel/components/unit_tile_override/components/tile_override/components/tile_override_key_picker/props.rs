use super::view::TileOverrideKeyPickerView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPickerCell;

/// Guards the hotkey picker so it is mounted only while editing (mounting it while
/// closed would attach its keyboard handling when it should be absent).
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideKeyPickerProps {
    pub visible: bool,
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}

impl From<&TileOverrideKeyPickerView> for TileOverrideKeyPickerProps {
    fn from(view: &TileOverrideKeyPickerView) -> Self {
        let TileOverrideKeyPickerView {
            visible,
            title,
            rows,
            on_pick,
            on_close,
        } = view.clone();
        Self {
            visible,
            title,
            rows,
            on_pick,
            on_close,
        }
    }
}

impl ddd::Props for TileOverrideKeyPickerProps {
    type View = TileOverrideKeyPickerView;
}
