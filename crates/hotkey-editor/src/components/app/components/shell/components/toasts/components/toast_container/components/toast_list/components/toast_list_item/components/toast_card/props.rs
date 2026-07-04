use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::ToastListItemProps;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastCardProps {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&ToastListItemProps> for ToastCardProps {
    fn from(props: &ToastListItemProps) -> Self {
        let record = props.record.clone();
        let on_remove = props.on_remove;
        Self { record, on_remove }
    }
}
