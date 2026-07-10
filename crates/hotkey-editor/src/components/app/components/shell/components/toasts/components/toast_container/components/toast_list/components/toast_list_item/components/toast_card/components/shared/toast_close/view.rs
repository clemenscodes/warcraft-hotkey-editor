use dioxus::prelude::*;

/// The published `View` contract mirroring [`ToastCloseProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ToastCloseView {
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl ddd::View for ToastCloseView {}
