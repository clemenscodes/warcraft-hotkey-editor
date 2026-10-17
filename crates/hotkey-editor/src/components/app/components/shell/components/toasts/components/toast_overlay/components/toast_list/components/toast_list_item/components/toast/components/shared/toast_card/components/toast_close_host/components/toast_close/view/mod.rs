use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastCloseView {
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl ddd::Layered for ToastCloseView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToastCloseView {}
