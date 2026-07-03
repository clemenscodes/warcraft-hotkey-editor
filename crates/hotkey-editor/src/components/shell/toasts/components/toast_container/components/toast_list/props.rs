use crate::components::shell::toasts::ToastRecord;
use crate::components::shell::toasts::components::toast_container::ToastContainerProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastListProps {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}

impl From<&ToastContainerProps> for ToastListProps {
    fn from(props: &ToastContainerProps) -> Self {
        let toasts = props.toasts.clone();
        let on_remove = props.on_remove;
        Self { toasts, on_remove }
    }
}
