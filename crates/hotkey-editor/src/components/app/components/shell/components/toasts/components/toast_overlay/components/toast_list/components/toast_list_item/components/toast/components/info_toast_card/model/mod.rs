use super::view::InfoToastCardView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoToastCardModel {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&InfoToastCardView> for InfoToastCardModel {
    fn from(view: &InfoToastCardView) -> Self {
        let InfoToastCardView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Model for InfoToastCardModel {
    type View = InfoToastCardView;
}
