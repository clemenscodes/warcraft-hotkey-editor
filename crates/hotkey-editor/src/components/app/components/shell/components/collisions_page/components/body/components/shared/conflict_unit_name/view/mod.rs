#[derive(Clone, PartialEq)]
pub struct ConflictUnitNameView {
    pub text: String,
}

impl ddd::Layered for ConflictUnitNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictUnitNameView {}
