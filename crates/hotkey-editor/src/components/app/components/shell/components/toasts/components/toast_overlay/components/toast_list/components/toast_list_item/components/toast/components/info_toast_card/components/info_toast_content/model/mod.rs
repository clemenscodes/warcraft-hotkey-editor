use super::view::InfoToastContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoToastContentModel {
    pub title: String,
    pub description: Option<String>,
}

impl From<&InfoToastContentView> for InfoToastContentModel {
    fn from(view: &InfoToastContentView) -> Self {
        let InfoToastContentView { title, description } = view.clone();
        Self { title, description }
    }
}

impl ddd::Model for InfoToastContentModel {
    type View = InfoToastContentView;
}
