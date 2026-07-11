use super::view::WarningToastTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarningToastTitleModel {
    pub title: String,
}

impl From<&WarningToastTitleView> for WarningToastTitleModel {
    fn from(view: &WarningToastTitleView) -> Self {
        let WarningToastTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Model for WarningToastTitleModel {
    type View = WarningToastTitleView;
}
