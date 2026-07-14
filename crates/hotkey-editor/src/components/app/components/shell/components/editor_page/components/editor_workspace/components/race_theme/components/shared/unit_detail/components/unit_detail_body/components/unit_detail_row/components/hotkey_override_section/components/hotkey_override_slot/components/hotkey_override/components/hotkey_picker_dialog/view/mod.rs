use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::KeyPickerCell;

/// The published `View` contract mirroring [`HotkeyPickerDialogDialogModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyPickerDialogDialogView {
    pub visible: bool,
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}

impl ddd::View for HotkeyPickerDialogDialogView {}
