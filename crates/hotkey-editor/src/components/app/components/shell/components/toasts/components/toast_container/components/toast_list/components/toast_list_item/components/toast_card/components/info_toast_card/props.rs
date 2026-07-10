use super::view::InfoToastCardView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoToastCardProps {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl From<&InfoToastCardView> for InfoToastCardProps {
    fn from(view: &InfoToastCardView) -> Self {
        let InfoToastCardView { record, on_remove } = view.clone();
        Self { record, on_remove }
    }
}

impl ddd::Props for InfoToastCardProps {
    type View = InfoToastCardView;
}
