use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastCloseView {
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl ddd::View for ToastCloseView {}
