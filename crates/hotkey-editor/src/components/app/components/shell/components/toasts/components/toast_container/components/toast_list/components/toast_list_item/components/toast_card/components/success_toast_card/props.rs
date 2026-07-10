use super::view::SuccessToastCardView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SuccessToastCardProps {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&SuccessToastCardView> for SuccessToastCardProps {
    fn from(view: &SuccessToastCardView) -> Self {
        let SuccessToastCardView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Props for SuccessToastCardProps {
    type View = SuccessToastCardView;
}
