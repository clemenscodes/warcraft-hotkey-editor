use super::view::SystemHotkeysDialogView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&SystemHotkeysDialogView> for SystemHotkeysDialogModel {
    fn from(view: &SystemHotkeysDialogView) -> Self {
        let SystemHotkeysDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for SystemHotkeysDialogModel {
    type View = SystemHotkeysDialogView;
}
