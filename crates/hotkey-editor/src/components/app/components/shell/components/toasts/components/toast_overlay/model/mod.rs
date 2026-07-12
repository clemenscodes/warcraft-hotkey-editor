use super::view::ToastOverlayView;
use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastOverlayModel {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}

impl From<&ToastOverlayView> for ToastOverlayModel {
    fn from(view: &ToastOverlayView) -> Self {
        let ToastOverlayView { toasts, on_remove } = view.clone();
        Self { toasts, on_remove }
    }
}

impl ddd::Model for ToastOverlayModel {
    type View = ToastOverlayView;
}
