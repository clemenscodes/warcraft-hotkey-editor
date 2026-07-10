/// The published `View` contract mirroring [`HighlightOverlayProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HighlightOverlayView {
    pub active: bool,
}

impl ddd::View for HighlightOverlayView {}
