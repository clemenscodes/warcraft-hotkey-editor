use super::view::GridLayoutEditorDialogView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutEditorDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&GridLayoutEditorDialogView> for GridLayoutEditorDialogModel {
    fn from(view: &GridLayoutEditorDialogView) -> Self {
        let GridLayoutEditorDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for GridLayoutEditorDialogModel {
    type View = GridLayoutEditorDialogView;
}
