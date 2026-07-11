use super::view::ErrorToastTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorToastTitleModel {
    pub title: String,
}

impl From<&ErrorToastTitleView> for ErrorToastTitleModel {
    fn from(view: &ErrorToastTitleView) -> Self {
        let ErrorToastTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Model for ErrorToastTitleModel {
    type View = ErrorToastTitleView;
}
