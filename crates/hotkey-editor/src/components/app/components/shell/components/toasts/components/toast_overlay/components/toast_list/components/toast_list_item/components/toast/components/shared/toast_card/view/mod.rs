use dioxus::prelude::*;

/// The published `View` contract mirroring [`ToastCardModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ToastCardView {
    pub icon: &'static str,
    pub title: String,
    pub description: Option<String>,
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl ddd::View for ToastCardView {}
