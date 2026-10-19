#[derive(Clone, PartialEq)]
pub struct DraggingSourceGhostView {
    pub active: bool,
}

impl ddd::Layered for DraggingSourceGhostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DraggingSourceGhostView {}
