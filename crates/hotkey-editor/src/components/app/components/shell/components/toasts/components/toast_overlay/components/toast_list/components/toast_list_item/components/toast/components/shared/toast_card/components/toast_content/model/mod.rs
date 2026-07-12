use super::view::ToastContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastContentModel {
    pub title: String,
    pub description: Option<String>,
}

impl From<&ToastContentView> for ToastContentModel {
    fn from(view: &ToastContentView) -> Self {
        let ToastContentView { title, description } = view.clone();
        Self { title, description }
    }
}

impl ddd::Model for ToastContentModel {
    type View = ToastContentView;
}
