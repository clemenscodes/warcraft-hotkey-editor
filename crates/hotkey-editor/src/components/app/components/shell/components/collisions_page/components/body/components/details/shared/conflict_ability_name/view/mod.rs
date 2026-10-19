#[derive(Clone, PartialEq)]
pub struct ConflictAbilityNameView {
    pub text: String,
}

impl ddd::Layered for ConflictAbilityNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictAbilityNameView {}
