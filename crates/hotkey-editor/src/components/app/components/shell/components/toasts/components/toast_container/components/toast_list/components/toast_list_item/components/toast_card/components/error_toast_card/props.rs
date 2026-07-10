use super::view::ErrorToastCardView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorToastCardProps {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&ErrorToastCardView> for ErrorToastCardProps {
    fn from(view: &ErrorToastCardView) -> Self {
        let ErrorToastCardView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Props for ErrorToastCardProps {
    type View = ErrorToastCardView;
}
