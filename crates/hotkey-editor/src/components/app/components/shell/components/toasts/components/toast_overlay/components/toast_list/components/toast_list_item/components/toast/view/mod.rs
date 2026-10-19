use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastView {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl ddd::Layered for ToastView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToastView {}
