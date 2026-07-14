use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastCloseHostView {
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl ddd::View for ToastCloseHostView {}
