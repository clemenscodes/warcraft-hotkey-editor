use super::view::ToastDescriptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastDescriptionModel {
    pub description: Option<String>,
}

impl From<&ToastDescriptionView> for ToastDescriptionModel {
    fn from(view: &ToastDescriptionView) -> Self {
        let ToastDescriptionView { description } = view.clone();
        Self { description }
    }
}

impl ddd::Model for ToastDescriptionModel {
    type View = ToastDescriptionView;
}
