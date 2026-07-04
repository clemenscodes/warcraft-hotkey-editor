use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::ToastCardProps;
use crate::components::app::components::shell::components::toasts::ToastType;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastIconProps {
    pub toast_type: ToastType,
}

impl From<&ToastCardProps> for ToastIconProps {
    fn from(props: &ToastCardProps) -> Self {
        let toast_type = props.record.toast_type();
        Self { toast_type }
    }
}
