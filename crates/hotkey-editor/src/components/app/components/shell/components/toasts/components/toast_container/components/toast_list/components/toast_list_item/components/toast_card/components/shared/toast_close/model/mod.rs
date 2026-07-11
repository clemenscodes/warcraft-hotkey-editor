use super::view::ToastCloseView;
use dioxus::prelude::*;

/// The close button's private internal model — the props it receives. Mirrors the
/// published [`ToastCloseView`] field-for-field; the `From<&View>` is the boundary
/// translation.
#[derive(Props, Clone, PartialEq)]
pub struct ToastCloseModel {
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl From<&ToastCloseView> for ToastCloseModel {
    fn from(view: &ToastCloseView) -> Self {
        let ToastCloseView { id, on_remove } = view.clone();
        Self { id, on_remove }
    }
}

impl ddd::Model for ToastCloseModel {
    type View = ToastCloseView;
}
