use super::view::ErrorToastContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorToastContentModel {
    pub title: String,
    pub description: Option<String>,
}

impl From<&ErrorToastContentView> for ErrorToastContentModel {
    fn from(view: &ErrorToastContentView) -> Self {
        let ErrorToastContentView { title, description } = view.clone();
        Self { title, description }
    }
}

impl ddd::Model for ErrorToastContentModel {
    type View = ErrorToastContentView;
}
