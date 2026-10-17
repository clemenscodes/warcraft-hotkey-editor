use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PreviewDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::Layered for PreviewDialogView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PreviewDialogView {}
