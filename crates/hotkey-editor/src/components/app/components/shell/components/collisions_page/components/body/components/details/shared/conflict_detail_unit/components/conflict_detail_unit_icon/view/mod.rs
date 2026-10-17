#[derive(Clone, PartialEq)]
pub struct ConflictDetailUnitIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::Layered for ConflictDetailUnitIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictDetailUnitIconView {}
