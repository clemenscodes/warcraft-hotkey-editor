use super::components::warning_toast_content::WarningToastContentProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_close::ToastCloseProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescriptionProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarningToastCardProps {
    pub title: String,
    pub description: ToastDescriptionProps,
    pub close: ToastCloseProps,
}

impl From<&WarningToastCardProps> for WarningToastContentProps {
    fn from(props: &WarningToastCardProps) -> Self {
        let title = props.title.clone();
        let description = props.description.clone();
        Self { title, description }
    }
}
