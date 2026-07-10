use super::view::WarningToastTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarningToastTitleProps {
    pub title: String,
}

impl From<&WarningToastTitleView> for WarningToastTitleProps {
    fn from(view: &WarningToastTitleView) -> Self {
        let WarningToastTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Props for WarningToastTitleProps {
    type View = WarningToastTitleView;
}
