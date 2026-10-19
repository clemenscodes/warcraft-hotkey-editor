#[derive(Clone, PartialEq)]
pub struct BlockedDropTargetOverlayView {
    pub active: bool,
}

impl ddd::Layered for BlockedDropTargetOverlayView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BlockedDropTargetOverlayView {}
