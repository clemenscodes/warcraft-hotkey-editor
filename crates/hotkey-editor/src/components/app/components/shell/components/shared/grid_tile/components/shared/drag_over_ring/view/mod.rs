/// The published `View` contract mirroring [`DragOverRingModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DragOverRingView {
    pub active: bool,
}

impl ddd::View for DragOverRingView {}
