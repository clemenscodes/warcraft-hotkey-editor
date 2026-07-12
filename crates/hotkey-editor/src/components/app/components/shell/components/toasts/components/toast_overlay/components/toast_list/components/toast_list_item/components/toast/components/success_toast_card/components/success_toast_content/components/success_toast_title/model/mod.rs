use super::view::SuccessToastTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SuccessToastTitleModel {
    pub title: String,
}

impl From<&SuccessToastTitleView> for SuccessToastTitleModel {
    fn from(view: &SuccessToastTitleView) -> Self {
        let SuccessToastTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Model for SuccessToastTitleModel {
    type View = SuccessToastTitleView;
}
