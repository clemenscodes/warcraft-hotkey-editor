use super::view::EmptyIslandDetailView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EmptyIslandDetailModel {
    #[props(into)]
    pub prompt: String,
}

impl From<&EmptyIslandDetailView> for EmptyIslandDetailModel {
    fn from(view: &EmptyIslandDetailView) -> Self {
        let EmptyIslandDetailView { prompt } = view.clone();
        Self { prompt }
    }
}

impl ddd::Model for EmptyIslandDetailModel {
    type View = EmptyIslandDetailView;
}
