use super::view::ToastListView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastListProps {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}

impl From<&ToastListView> for ToastListProps {
    fn from(view: &ToastListView) -> Self {
        let ToastListView { toasts, on_remove } = view.clone();
        Self { toasts, on_remove }
    }
}

impl ddd::Props for ToastListProps {
    type View = ToastListView;
}
