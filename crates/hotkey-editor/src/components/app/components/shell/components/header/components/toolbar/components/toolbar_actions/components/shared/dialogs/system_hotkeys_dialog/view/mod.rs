use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SystemHotkeysDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::Layered for SystemHotkeysDialogView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysDialogView {}
