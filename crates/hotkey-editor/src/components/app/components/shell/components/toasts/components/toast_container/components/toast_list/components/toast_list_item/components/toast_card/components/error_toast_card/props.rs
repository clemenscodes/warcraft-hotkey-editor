use super::components::error_toast_content::ErrorToastContentProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_close::ToastCloseProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescriptionProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorToastCardProps {
    pub title: String,
    pub description: ToastDescriptionProps,
    pub close: ToastCloseProps,
}

impl From<&ErrorToastCardProps> for ErrorToastContentProps {
    fn from(props: &ErrorToastCardProps) -> Self {
        let title = props.title.clone();
        let description = props.description.clone();
        Self { title, description }
    }
}
