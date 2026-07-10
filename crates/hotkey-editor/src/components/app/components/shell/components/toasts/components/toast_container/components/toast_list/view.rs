use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`ToastListProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ToastListView {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}

impl ddd::View for ToastListView {}
