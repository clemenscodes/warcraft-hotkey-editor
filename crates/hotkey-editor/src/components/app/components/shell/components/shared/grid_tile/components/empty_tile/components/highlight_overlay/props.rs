use super::view::HighlightOverlayView;
use dioxus::prelude::*;

/// Mounts only on the one coordinate a mini grid marks; every other empty slot leaves
/// `active` false and early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct HighlightOverlayProps {
    pub active: bool,
}

impl From<&HighlightOverlayView> for HighlightOverlayProps {
    fn from(view: &HighlightOverlayView) -> Self {
        let HighlightOverlayView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Props for HighlightOverlayProps {
    type View = HighlightOverlayView;
}
