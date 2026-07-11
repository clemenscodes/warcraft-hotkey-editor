/// The published `View` contract mirroring [`BlockedDropTargetOverlayModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BlockedDropTargetOverlayView {
    pub active: bool,
}

impl ddd::View for BlockedDropTargetOverlayView {}
