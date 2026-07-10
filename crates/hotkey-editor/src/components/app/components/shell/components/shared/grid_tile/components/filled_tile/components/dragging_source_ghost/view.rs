/// The published `View` contract mirroring [`DraggingSourceGhostProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DraggingSourceGhostView {
    pub active: bool,
}

impl ddd::View for DraggingSourceGhostView {}
