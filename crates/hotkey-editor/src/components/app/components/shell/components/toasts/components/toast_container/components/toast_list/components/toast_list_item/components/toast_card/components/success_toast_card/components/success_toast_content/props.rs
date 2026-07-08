use super::components::success_toast_title::SuccessToastTitleProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescriptionProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SuccessToastContentProps {
    pub title: String,
    pub description: ToastDescriptionProps,
}

impl From<&SuccessToastContentProps> for SuccessToastTitleProps {
    fn from(props: &SuccessToastContentProps) -> Self {
        let title = props.title.clone();
        Self { title }
    }
}
