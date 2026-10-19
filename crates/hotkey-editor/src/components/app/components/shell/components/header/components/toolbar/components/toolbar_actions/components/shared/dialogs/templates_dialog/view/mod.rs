use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TemplatesDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::Layered for TemplatesDialogView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TemplatesDialogView {}
