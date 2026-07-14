use super::view::HighlightOverlayView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HighlightOverlayModel {
    pub active: bool,
}

impl From<&HighlightOverlayView> for HighlightOverlayModel {
    fn from(view: &HighlightOverlayView) -> Self {
        let HighlightOverlayView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Model for HighlightOverlayModel {
    type View = HighlightOverlayView;
}
