use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::View for SearchDialogView {}
