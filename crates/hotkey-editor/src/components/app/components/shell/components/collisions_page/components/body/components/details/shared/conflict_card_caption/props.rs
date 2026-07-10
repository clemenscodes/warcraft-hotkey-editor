use super::view::ConflictCardCaptionView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictCardCaptionProps {
    #[props(into)]
    pub text: String,
}

impl From<&ConflictCardCaptionView> for ConflictCardCaptionProps {
    fn from(view: &ConflictCardCaptionView) -> Self {
        let ConflictCardCaptionView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for ConflictCardCaptionProps {
    type View = ConflictCardCaptionView;
}
