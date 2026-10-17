use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastCardView {
    pub icon: &'static str,
    pub title: String,
    pub description: Option<String>,
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl ddd::Layered for ToastCardView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToastCardView {}
