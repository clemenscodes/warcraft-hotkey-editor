use super::view::ResolvePageView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResolvePageModel {
    pub entry: Option<String>,
}

impl From<&ResolvePageView> for ResolvePageModel {
    fn from(view: &ResolvePageView) -> Self {
        let ResolvePageView { entry } = view.clone();
        Self { entry }
    }
}

impl ddd::Model for ResolvePageModel {
    type View = ResolvePageView;
}
