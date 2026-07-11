use super::view::SystemKeyPickerDialogPanelView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The system key picker's bordered box: the header row above the scrolling board body,
/// wrapped in the library `DialogContent` (which carries no project class — this panel's
/// own classed `div` is the box). It carries the header's title and close handler and
/// the raw board values (both columns plus the pick and Escape handlers) it threads to
/// the board body.
#[derive(Props, Clone, PartialEq)]
pub struct SystemKeyPickerDialogPanelModel {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub board_on_close: EventHandler<()>,
}

impl From<&SystemKeyPickerDialogPanelView> for SystemKeyPickerDialogPanelModel {
    fn from(view: &SystemKeyPickerDialogPanelView) -> Self {
        let SystemKeyPickerDialogPanelView {
            title,
            on_close,
            columns,
            on_pick,
            board_on_close,
        } = view.clone();
        Self {
            title,
            on_close,
            columns,
            on_pick,
            board_on_close,
        }
    }
}

impl ddd::Model for SystemKeyPickerDialogPanelModel {
    type View = SystemKeyPickerDialogPanelView;
}
