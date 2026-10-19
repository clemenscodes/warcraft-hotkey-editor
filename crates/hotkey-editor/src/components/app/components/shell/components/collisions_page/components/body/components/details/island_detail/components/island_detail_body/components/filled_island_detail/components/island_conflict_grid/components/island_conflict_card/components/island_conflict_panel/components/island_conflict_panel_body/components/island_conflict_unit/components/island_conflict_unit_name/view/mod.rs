#[derive(Clone, PartialEq)]
pub struct IslandConflictUnitNameView {
    pub text: String,
}

impl ddd::Layered for IslandConflictUnitNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandConflictUnitNameView {}
