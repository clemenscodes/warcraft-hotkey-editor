#[derive(Clone, PartialEq)]
pub struct MutedManaValueView {
    pub text: String,
}

impl ddd::Layered for MutedManaValueView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MutedManaValueView {}
