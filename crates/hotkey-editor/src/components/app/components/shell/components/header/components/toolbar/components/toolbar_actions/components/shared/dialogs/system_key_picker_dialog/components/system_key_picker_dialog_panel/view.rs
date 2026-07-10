use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The published `View` contract mirroring [`SystemKeyPickerDialogPanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemKeyPickerDialogPanelView {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub board_on_close: EventHandler<()>,
}

impl ddd::View for SystemKeyPickerDialogPanelView {}
