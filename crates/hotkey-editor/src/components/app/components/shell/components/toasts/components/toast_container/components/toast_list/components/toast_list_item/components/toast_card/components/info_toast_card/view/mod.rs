use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`InfoToastCardModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoToastCardView {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl ddd::View for InfoToastCardView {}
