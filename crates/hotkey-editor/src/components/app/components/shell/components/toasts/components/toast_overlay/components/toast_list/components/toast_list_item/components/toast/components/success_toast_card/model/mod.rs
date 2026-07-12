use super::view::SuccessToastCardView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SuccessToastCardModel {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&SuccessToastCardView> for SuccessToastCardModel {
    fn from(view: &SuccessToastCardView) -> Self {
        let SuccessToastCardView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Model for SuccessToastCardModel {
    type View = SuccessToastCardView;
}
