use super::view::HotkeyPickerDialogDialogView;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::KeyPickerCell;

/// Guards the hotkey picker so it is mounted only while editing (mounting it while
/// closed would attach its keyboard handling when it should be absent).
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyPickerDialogDialogModel {
    pub visible: bool,
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}

impl From<&HotkeyPickerDialogDialogView> for HotkeyPickerDialogDialogModel {
    fn from(view: &HotkeyPickerDialogDialogView) -> Self {
        let HotkeyPickerDialogDialogView {
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

impl ddd::Model for HotkeyPickerDialogDialogModel {
    type View = HotkeyPickerDialogDialogView;
}
