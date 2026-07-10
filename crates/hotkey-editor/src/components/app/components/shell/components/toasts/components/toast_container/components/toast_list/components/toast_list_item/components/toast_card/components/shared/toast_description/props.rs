use super::view::ToastDescriptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastDescriptionProps {
    pub description: Option<String>,
}

impl From<&ToastDescriptionView> for ToastDescriptionProps {
    fn from(view: &ToastDescriptionView) -> Self {
        let ToastDescriptionView { description } = view.clone();
        Self { description }
    }
}

impl ddd::Props for ToastDescriptionProps {
    type View = ToastDescriptionView;
}
