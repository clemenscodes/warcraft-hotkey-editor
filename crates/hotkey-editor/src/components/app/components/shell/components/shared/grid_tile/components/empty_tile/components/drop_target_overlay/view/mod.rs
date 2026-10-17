#[derive(Clone, PartialEq)]
pub struct DropTargetOverlayView {
    pub active: bool,
}

impl ddd::Layered for DropTargetOverlayView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DropTargetOverlayView {}
