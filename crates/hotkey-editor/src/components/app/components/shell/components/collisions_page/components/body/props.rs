use super::model::ContentModel;
use super::view::BodyView;
use dioxus::prelude::*;

/// The dispatcher's input: the shaped active content for the current kind and state.
#[derive(Props, Clone, PartialEq)]
pub struct BodyProps {
    pub content: ContentModel,
}

impl From<&BodyView> for BodyProps {
    fn from(view: &BodyView) -> Self {
        let BodyView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Props for BodyProps {
    type View = BodyView;
}
