use super::components::warning_toast_title::WarningToastTitleProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescriptionProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarningToastContentProps {
    pub title: String,
    pub description: ToastDescriptionProps,
}

impl From<&WarningToastContentProps> for WarningToastTitleProps {
    fn from(props: &WarningToastContentProps) -> Self {
        let title = props.title.clone();
        Self { title }
    }
}
