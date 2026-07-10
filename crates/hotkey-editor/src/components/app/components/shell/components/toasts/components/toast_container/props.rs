use super::view::ToastContainerView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastContainerProps {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}

impl From<&ToastContainerView> for ToastContainerProps {
    fn from(view: &ToastContainerView) -> Self {
        let ToastContainerView { toasts, on_remove } = view.clone();
        Self { toasts, on_remove }
    }
}

impl ddd::Props for ToastContainerProps {
    type View = ToastContainerView;
}
