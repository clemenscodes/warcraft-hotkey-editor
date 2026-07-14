use super::view::BodyView;
use super::view::ContentModel;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BodyModel {
    pub content: ContentModel,
}

impl From<&BodyView> for BodyModel {
    fn from(view: &BodyView) -> Self {
        let BodyView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Model for BodyModel {
    type View = BodyView;
}
