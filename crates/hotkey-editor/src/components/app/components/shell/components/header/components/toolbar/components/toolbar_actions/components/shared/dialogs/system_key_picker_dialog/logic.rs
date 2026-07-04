use super::components::system_key_picker_board::SystemKeyPickerBoard;
use super::hooks::SystemKeyPickerModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

impl From<&SystemKeyPickerModel> for DialogProps {
    fn from(model: &SystemKeyPickerModel) -> Self {
        let open = model.open;
        let title = model.title.clone();
        let board = model.board.clone();
        let children = rsx! {
            SystemKeyPickerBoard { ..board }
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
