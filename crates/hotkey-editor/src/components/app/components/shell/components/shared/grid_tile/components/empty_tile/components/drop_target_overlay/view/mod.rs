/// The published `View` contract mirroring [`DropTargetOverlayModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DropTargetOverlayView {
    pub active: bool,
}

impl ddd::View for DropTargetOverlayView {}
