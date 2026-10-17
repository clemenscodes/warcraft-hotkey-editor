use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastCloseHostView {
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl ddd::Layered for ToastCloseHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToastCloseHostView {}
