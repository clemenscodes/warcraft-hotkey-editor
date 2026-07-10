use super::view::WarningToastContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarningToastContentProps {
    pub title: String,
    pub description: Option<String>,
}

impl From<&WarningToastContentView> for WarningToastContentProps {
    fn from(view: &WarningToastContentView) -> Self {
        let WarningToastContentView { title, description } = view.clone();
        Self { title, description }
    }
}

impl ddd::Props for WarningToastContentProps {
    type View = WarningToastContentView;
}
