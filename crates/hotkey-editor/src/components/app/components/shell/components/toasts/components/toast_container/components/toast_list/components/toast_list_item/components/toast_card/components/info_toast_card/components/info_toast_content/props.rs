use super::view::InfoToastContentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoToastContentProps {
    pub title: String,
    pub description: Option<String>,
}

impl From<&InfoToastContentView> for InfoToastContentProps {
    fn from(view: &InfoToastContentView) -> Self {
        let InfoToastContentView { title, description } = view.clone();
        Self { title, description }
    }
}

impl ddd::Props for InfoToastContentProps {
    type View = InfoToastContentView;
}
