use super::view::WarningToastContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarningToastContentModel {
    pub title: String,
    pub description: Option<String>,
}

impl From<&WarningToastContentView> for WarningToastContentModel {
    fn from(view: &WarningToastContentView) -> Self {
        let WarningToastContentView { title, description } = view.clone();
        Self { title, description }
    }
}

impl ddd::Model for WarningToastContentModel {
    type View = WarningToastContentView;
}
