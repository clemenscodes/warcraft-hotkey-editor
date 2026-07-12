use super::view::WarningToastCardView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarningToastCardModel {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&WarningToastCardView> for WarningToastCardModel {
    fn from(view: &WarningToastCardView) -> Self {
        let WarningToastCardView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Model for WarningToastCardModel {
    type View = WarningToastCardView;
}
