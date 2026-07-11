use super::view::ToastListView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastListModel {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}

impl From<&ToastListView> for ToastListModel {
    fn from(view: &ToastListView) -> Self {
        let ToastListView { toasts, on_remove } = view.clone();
        Self { toasts, on_remove }
    }
}

impl ddd::Model for ToastListModel {
    type View = ToastListView;
}
