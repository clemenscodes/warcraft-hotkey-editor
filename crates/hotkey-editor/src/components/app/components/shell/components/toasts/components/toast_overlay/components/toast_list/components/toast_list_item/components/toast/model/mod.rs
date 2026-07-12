use super::view::ToastView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastModel {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&ToastView> for ToastModel {
    fn from(view: &ToastView) -> Self {
        let ToastView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Model for ToastModel {
    type View = ToastView;
}
