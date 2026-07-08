use super::components::info_toast_content::InfoToastContentProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_close::ToastCloseProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescriptionProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoToastCardProps {
    pub title: String,
    pub description: ToastDescriptionProps,
    pub close: ToastCloseProps,
}

impl From<&InfoToastCardProps> for InfoToastContentProps {
    fn from(props: &InfoToastCardProps) -> Self {
        let title = props.title.clone();
        let description = props.description.clone();
        Self { title, description }
    }
}
