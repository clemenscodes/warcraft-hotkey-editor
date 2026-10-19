use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastListView {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}

impl ddd::Layered for ToastListView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToastListView {}
