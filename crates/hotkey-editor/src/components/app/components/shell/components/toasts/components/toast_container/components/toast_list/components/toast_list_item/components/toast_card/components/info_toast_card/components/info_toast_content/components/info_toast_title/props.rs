use super::view::InfoToastTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoToastTitleProps {
    pub title: String,
}

impl From<&InfoToastTitleView> for InfoToastTitleProps {
    fn from(view: &InfoToastTitleView) -> Self {
        let InfoToastTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Props for InfoToastTitleProps {
    type View = InfoToastTitleView;
}
