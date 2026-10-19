#[derive(Clone, PartialEq)]
pub struct ConflictUnitIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::Layered for ConflictUnitIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictUnitIconView {}
