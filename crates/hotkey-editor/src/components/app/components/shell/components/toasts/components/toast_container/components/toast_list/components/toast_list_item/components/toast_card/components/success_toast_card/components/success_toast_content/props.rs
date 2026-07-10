use super::view::SuccessToastContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SuccessToastContentProps {
    pub title: String,
    pub description: Option<String>,
}

impl From<&SuccessToastContentView> for SuccessToastContentProps {
    fn from(view: &SuccessToastContentView) -> Self {
        let SuccessToastContentView { title, description } = view.clone();
        Self { title, description }
    }
}

impl ddd::Props for SuccessToastContentProps {
    type View = SuccessToastContentView;
}
