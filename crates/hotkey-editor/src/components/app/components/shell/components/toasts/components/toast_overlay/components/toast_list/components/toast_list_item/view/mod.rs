use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastListItemView {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl ddd::View for ToastListItemView {}
