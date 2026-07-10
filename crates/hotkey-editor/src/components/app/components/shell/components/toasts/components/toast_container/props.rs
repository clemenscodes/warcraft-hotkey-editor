use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastContainerProps {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}
