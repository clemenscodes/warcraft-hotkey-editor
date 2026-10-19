#[derive(Clone, PartialEq)]
pub struct MutedStatValueView {
    pub text: String,
}

impl ddd::Layered for MutedStatValueView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MutedStatValueView {}
