use super::view::PreviewDialogView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&PreviewDialogView> for PreviewDialogModel {
    fn from(view: &PreviewDialogView) -> Self {
        let PreviewDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for PreviewDialogModel {
    type View = PreviewDialogView;
}
