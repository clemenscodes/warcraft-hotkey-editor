use crate::components::shell::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastListItemProps {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}
