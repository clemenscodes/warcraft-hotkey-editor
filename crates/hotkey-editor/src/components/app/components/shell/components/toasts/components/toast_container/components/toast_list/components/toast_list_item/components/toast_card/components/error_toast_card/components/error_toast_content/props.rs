use super::components::error_toast_title::ErrorToastTitleProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescriptionProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorToastContentProps {
    pub title: String,
    pub description: ToastDescriptionProps,
}

impl From<&ErrorToastContentProps> for ErrorToastTitleProps {
    fn from(props: &ErrorToastContentProps) -> Self {
        let title = props.title.clone();
        Self { title }
    }
}
