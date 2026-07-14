#[derive(Clone, PartialEq)]
pub struct BlockedDropTargetOverlayView {
    pub active: bool,
}

impl ddd::View for BlockedDropTargetOverlayView {}
