use super::view::ErrorToastTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorToastTitleProps {
    pub title: String,
}

impl From<&ErrorToastTitleView> for ErrorToastTitleProps {
    fn from(view: &ErrorToastTitleView) -> Self {
        let ErrorToastTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Props for ErrorToastTitleProps {
    type View = ErrorToastTitleView;
}
