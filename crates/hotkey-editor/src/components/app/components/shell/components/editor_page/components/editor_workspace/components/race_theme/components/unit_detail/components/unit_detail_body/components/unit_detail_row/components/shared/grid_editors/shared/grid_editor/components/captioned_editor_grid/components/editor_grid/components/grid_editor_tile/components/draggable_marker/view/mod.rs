#[derive(Clone, PartialEq)]
pub struct DraggableMarkerView {
    pub active: bool,
}

impl ddd::Layered for DraggableMarkerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DraggableMarkerView {}
