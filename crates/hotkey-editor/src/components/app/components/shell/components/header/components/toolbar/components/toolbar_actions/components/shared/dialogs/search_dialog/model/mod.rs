use super::view::SearchDialogView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&SearchDialogView> for SearchDialogModel {
    fn from(view: &SearchDialogView) -> Self {
        let SearchDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for SearchDialogModel {
    type View = SearchDialogView;
}
