use super::view::ToastCloseView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastCloseProps {
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl From<&ToastCloseView> for ToastCloseProps {
    fn from(view: &ToastCloseView) -> Self {
        let ToastCloseView { id, on_remove } = view.clone();
        Self { id, on_remove }
    }
}

impl ddd::Props for ToastCloseProps {
    type View = ToastCloseView;
}
