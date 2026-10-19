#[derive(Clone, PartialEq)]
pub struct MutedStatGainView {
    pub text: String,
}

impl ddd::Layered for MutedStatGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MutedStatGainView {}
