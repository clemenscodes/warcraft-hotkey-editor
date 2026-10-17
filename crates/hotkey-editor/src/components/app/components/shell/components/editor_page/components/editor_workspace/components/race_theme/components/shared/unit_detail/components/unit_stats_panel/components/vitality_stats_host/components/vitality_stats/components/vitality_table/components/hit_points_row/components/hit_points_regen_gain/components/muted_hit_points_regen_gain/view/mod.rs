#[derive(Clone, PartialEq)]
pub struct MutedHitPointsRegenGainView {
    pub text: String,
    pub condition: Option<&'static str>,
}

impl ddd::Layered for MutedHitPointsRegenGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MutedHitPointsRegenGainView {}
