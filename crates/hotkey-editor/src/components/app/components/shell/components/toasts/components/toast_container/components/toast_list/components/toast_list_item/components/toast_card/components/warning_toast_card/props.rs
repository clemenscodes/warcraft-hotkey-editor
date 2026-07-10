use super::view::WarningToastCardView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarningToastCardProps {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&WarningToastCardView> for WarningToastCardProps {
    fn from(view: &WarningToastCardView) -> Self {
        let WarningToastCardView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Props for WarningToastCardProps {
    type View = WarningToastCardView;
}
