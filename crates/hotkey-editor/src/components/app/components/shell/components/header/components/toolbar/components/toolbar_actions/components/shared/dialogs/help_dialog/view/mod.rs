use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HelpDialogView {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl ddd::Layered for HelpDialogView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpDialogView {}
