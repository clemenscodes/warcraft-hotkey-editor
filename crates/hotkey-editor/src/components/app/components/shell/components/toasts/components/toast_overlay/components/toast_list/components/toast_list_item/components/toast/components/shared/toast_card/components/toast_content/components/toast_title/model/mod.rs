use super::view::ToastTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastTitleModel {
    pub title: String,
}

impl From<&ToastTitleView> for ToastTitleModel {
    fn from(view: &ToastTitleView) -> Self {
        let ToastTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Model for ToastTitleModel {
    type View = ToastTitleView;
}
