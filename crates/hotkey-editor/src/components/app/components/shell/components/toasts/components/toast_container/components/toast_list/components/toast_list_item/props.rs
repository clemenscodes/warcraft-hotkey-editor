use super::view::ToastListItemView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastListItemProps {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&ToastListItemView> for ToastListItemProps {
    fn from(view: &ToastListItemView) -> Self {
        let ToastListItemView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Props for ToastListItemProps {
    type View = ToastListItemView;
}
