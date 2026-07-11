use super::view::ToastCardView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastCardModel {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&ToastCardView> for ToastCardModel {
    fn from(view: &ToastCardView) -> Self {
        let ToastCardView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Model for ToastCardModel {
    type View = ToastCardView;
}
