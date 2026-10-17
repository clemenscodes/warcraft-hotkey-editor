use super::view::ToastCloseHostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastCloseHostModel {
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl From<&ToastCloseHostView> for ToastCloseHostModel {
    fn from(view: &ToastCloseHostView) -> Self {
        let ToastCloseHostView { id, on_remove } = view.clone();
        Self { id, on_remove }
    }
}

impl ddd::Layered for ToastCloseHostModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ToastCloseHostModel {
    type View = ToastCloseHostView;
}
