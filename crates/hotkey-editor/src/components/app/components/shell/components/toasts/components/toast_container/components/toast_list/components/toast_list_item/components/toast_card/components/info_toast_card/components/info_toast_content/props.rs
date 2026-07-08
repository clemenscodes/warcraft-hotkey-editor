use super::components::info_toast_title::InfoToastTitleProps;
use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_description::ToastDescriptionProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoToastContentProps {
    pub title: String,
    pub description: ToastDescriptionProps,
}

impl From<&InfoToastContentProps> for InfoToastTitleProps {
    fn from(props: &InfoToastContentProps) -> Self {
        let title = props.title.clone();
        Self { title }
    }
}
