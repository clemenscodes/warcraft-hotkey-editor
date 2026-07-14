use super::view::HelpDialogView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&HelpDialogView> for HelpDialogModel {
    fn from(view: &HelpDialogView) -> Self {
        let HelpDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for HelpDialogModel {
    type View = HelpDialogView;
}
