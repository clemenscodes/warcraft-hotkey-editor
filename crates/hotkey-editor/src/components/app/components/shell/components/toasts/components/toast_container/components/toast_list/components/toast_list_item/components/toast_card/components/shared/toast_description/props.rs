use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::ToastCardProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastDescriptionProps {
    pub description: Option<String>,
}

impl From<&ToastCardProps> for ToastDescriptionProps {
    fn from(props: &ToastCardProps) -> Self {
        let description = props.record.description();
        Self { description }
    }
}
