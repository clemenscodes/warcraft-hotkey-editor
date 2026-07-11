use super::view::ConflictCardCaptionView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictCardCaptionModel {
    #[props(into)]
    pub text: String,
}

impl From<&ConflictCardCaptionView> for ConflictCardCaptionModel {
    fn from(view: &ConflictCardCaptionView) -> Self {
        let ConflictCardCaptionView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for ConflictCardCaptionModel {
    type View = ConflictCardCaptionView;
}
