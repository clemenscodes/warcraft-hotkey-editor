use crate::components::shell::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::ToastCardProps;
use crate::components::shell::toasts::ToastType;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastContentProps {
    pub title: String,
    pub description: Option<String>,
    pub toast_type: ToastType,
}

impl From<&ToastCardProps> for ToastContentProps {
    fn from(props: &ToastCardProps) -> Self {
        let record = &props.record;
        let title = record.title().to_string();
        let description = record.description();
        let toast_type = record.toast_type();
        Self {
            title,
            description,
            toast_type,
        }
    }
}
