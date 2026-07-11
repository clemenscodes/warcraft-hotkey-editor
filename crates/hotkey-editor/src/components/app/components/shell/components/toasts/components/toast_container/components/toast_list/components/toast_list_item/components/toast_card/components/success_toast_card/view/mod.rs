use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`SuccessToastCardModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SuccessToastCardView {
    pub record: ToastRecord,
    pub on_remove: Callback<usize>,
}

impl ddd::View for SuccessToastCardView {}
