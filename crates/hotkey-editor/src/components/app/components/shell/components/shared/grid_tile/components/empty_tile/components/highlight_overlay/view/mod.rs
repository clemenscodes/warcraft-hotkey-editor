#[derive(Clone, PartialEq)]
pub struct HighlightOverlayView {
    pub active: bool,
}

impl ddd::Layered for HighlightOverlayView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HighlightOverlayView {}
