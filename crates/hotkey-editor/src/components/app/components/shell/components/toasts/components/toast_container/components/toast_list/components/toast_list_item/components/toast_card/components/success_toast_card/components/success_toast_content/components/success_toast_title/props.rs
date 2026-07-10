use super::view::SuccessToastTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SuccessToastTitleProps {
    pub title: String,
}

impl From<&SuccessToastTitleView> for SuccessToastTitleProps {
    fn from(view: &SuccessToastTitleView) -> Self {
        let SuccessToastTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Props for SuccessToastTitleProps {
    type View = SuccessToastTitleView;
}
