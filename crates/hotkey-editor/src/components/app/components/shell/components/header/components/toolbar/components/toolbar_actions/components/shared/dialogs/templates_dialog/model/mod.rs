use super::view::TemplatesDialogView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&TemplatesDialogView> for TemplatesDialogModel {
    fn from(view: &TemplatesDialogView) -> Self {
        let TemplatesDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for TemplatesDialogModel {
    type View = TemplatesDialogView;
}
