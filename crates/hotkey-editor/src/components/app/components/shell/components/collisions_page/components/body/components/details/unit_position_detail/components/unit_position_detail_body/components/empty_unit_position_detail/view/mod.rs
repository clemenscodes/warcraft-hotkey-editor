#[derive(Clone, PartialEq)]
pub struct EmptyUnitPositionDetailView {
    pub prompt: String,
}

impl ddd::Layered for EmptyUnitPositionDetailView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EmptyUnitPositionDetailView {}
