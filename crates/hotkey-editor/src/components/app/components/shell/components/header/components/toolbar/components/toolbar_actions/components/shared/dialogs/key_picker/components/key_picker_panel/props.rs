use super::view::KeyPickerPanelView;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The key picker's bordered box: the title and close handler its header row shows,
/// above the scrolling board body (the columns of keys with the pick and keyboard-
/// dismiss handlers). Wrapped in the library `DialogContent` (which carries no project
/// class — this panel's own classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerPanelProps {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_board_close: EventHandler<()>,
}

impl From<&KeyPickerPanelView> for KeyPickerPanelProps {
    fn from(view: &KeyPickerPanelView) -> Self {
        let KeyPickerPanelView {
            title,
            on_close,
            columns,
            on_pick,
            on_board_close,
        } = view.clone();
        Self {
            title,
            on_close,
            columns,
            on_pick,
            on_board_close,
        }
    }
}

impl ddd::Props for KeyPickerPanelProps {
    type View = KeyPickerPanelView;
}
