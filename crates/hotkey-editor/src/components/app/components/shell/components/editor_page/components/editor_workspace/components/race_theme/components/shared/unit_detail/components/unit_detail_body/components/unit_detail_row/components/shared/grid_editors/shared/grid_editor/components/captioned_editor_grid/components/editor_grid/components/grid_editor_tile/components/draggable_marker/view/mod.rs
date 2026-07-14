#[derive(Clone, PartialEq)]
pub struct DraggableMarkerView {
    pub active: bool,
}

impl ddd::View for DraggableMarkerView {}
