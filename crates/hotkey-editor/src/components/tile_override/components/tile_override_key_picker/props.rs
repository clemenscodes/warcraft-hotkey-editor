use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use crate::components::dialogs::key_picker::{KeyPickerCell, KeyPickerProps};

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

impl From<&TileOverrideKeyPickerProps> for KeyPickerProps {
    fn from(props: &TileOverrideKeyPickerProps) -> Self {
        let title = props.title.clone();
        let rows = props.rows.clone();
        let open = true;
        let allow_conflict_pick = false;
        let on_pick = props.on_pick;
        let on_close = props.on_close;
        Self {
            title,
            rows,
            open,
            allow_conflict_pick,
            on_pick,
            on_close,
        }
    }
}
