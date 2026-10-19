#[derive(Clone, PartialEq)]
pub struct ConflictUnitIconHostView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::Layered for ConflictUnitIconHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictUnitIconHostView {}
