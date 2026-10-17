#[derive(Clone, PartialEq)]
pub struct MutedManaRegenGainView {
    pub text: String,
}

impl ddd::Layered for MutedManaRegenGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MutedManaRegenGainView {}
