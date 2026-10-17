use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GridLayoutEditorDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::Layered for GridLayoutEditorDialogView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for GridLayoutEditorDialogView {}
