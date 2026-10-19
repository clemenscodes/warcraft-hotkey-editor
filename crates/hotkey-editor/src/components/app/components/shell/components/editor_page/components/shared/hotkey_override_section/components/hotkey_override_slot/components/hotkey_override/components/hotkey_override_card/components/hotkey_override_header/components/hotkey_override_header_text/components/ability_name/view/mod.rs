#[derive(Clone, PartialEq)]
pub struct AbilityNameView {
    pub text: String,
}

impl ddd::Layered for AbilityNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AbilityNameView {}
