use crate::components::shell::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::toast_content::ToastContentProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastDescriptionProps {
    pub description: Option<String>,
}

impl From<&ToastContentProps> for ToastDescriptionProps {
    fn from(props: &ToastContentProps) -> Self {
        let description = props.description.clone();
        Self { description }
    }
}
