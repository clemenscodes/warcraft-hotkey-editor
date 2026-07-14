use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HelpDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::View for HelpDialogView {}
