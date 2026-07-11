use super::view::SuccessToastContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SuccessToastContentModel {
    pub title: String,
    pub description: Option<String>,
}

impl From<&SuccessToastContentView> for SuccessToastContentModel {
    fn from(view: &SuccessToastContentView) -> Self {
        let SuccessToastContentView { title, description } = view.clone();
        Self { title, description }
    }
}

impl ddd::Model for SuccessToastContentModel {
    type View = SuccessToastContentView;
}
