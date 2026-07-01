use super::components::key_picker_board::KeyPickerBoard;
use super::hooks::KeyPickerModel;
use crate::components::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

impl From<&KeyPickerModel> for DialogProps {
    fn from(model: &KeyPickerModel) -> Self {
        let open = model.open;
        let title = model.title.clone();
        let board = model.board.clone();
        let children = rsx! {
            KeyPickerBoard { ..board }
        };
        Self {
            open,
            title,
            children,
            footer: None,
            on_open_change: None,
        }
    }
}
