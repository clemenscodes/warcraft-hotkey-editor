/// The published `View` contract mirroring [`DropTargetOverlayProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DropTargetOverlayView {
    pub active: bool,
}

impl ddd::View for DropTargetOverlayView {}
