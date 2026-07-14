#[derive(Clone, PartialEq)]
pub struct DropTargetOverlayView {
    pub active: bool,
}

impl ddd::View for DropTargetOverlayView {}
