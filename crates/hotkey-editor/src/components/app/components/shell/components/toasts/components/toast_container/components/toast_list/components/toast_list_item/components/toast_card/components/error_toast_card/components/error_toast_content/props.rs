use super::view::ErrorToastContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorToastContentProps {
    pub title: String,
    pub description: Option<String>,
}

impl From<&ErrorToastContentView> for ErrorToastContentProps {
    fn from(view: &ErrorToastContentView) -> Self {
        let ErrorToastContentView { title, description } = view.clone();
        Self { title, description }
    }
}

impl ddd::Props for ErrorToastContentProps {
    type View = ErrorToastContentView;
}
