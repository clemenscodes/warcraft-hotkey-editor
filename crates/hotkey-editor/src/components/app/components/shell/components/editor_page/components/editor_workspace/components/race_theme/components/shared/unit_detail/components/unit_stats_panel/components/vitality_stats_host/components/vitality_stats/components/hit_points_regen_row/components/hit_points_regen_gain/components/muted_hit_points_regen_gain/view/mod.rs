#[derive(Clone, PartialEq)]
pub struct MutedHitPointsRegenGainView {
    pub text: String,
}

impl ddd::Layered for MutedHitPointsRegenGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MutedHitPointsRegenGainView {}
