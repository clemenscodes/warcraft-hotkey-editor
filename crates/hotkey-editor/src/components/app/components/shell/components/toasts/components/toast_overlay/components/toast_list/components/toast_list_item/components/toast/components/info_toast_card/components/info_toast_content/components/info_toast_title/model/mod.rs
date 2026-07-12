use super::view::InfoToastTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoToastTitleModel {
    pub title: String,
}

impl From<&InfoToastTitleView> for InfoToastTitleModel {
    fn from(view: &InfoToastTitleView) -> Self {
        let InfoToastTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Model for InfoToastTitleModel {
    type View = InfoToastTitleView;
}
