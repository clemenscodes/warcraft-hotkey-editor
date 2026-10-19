#[derive(Clone, PartialEq)]
pub struct DragOverRingView {
    pub active: bool,
}

impl ddd::Layered for DragOverRingView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DragOverRingView {}
