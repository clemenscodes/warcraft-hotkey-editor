/// The published `View` contract mirroring [`DraggableMarkerProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DraggableMarkerView {
    pub active: bool,
}

impl ddd::View for DraggableMarkerView {}
