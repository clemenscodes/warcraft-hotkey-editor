use crate::components::shell::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::ToastCardProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastCloseProps {
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl From<&ToastCardProps> for ToastCloseProps {
    fn from(props: &ToastCardProps) -> Self {
        let id = props.record.id();
        let on_remove = props.on_remove;
        Self { id, on_remove }
    }
}
