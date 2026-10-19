use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchDialogScrimView {
    pub onclick: EventHandler<MouseEvent>,
    pub leaving: bool,
}

impl ddd::Layered for SearchDialogScrimView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SearchDialogScrimView {}
