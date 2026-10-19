#[derive(Clone, PartialEq)]
pub struct MutedHitPointsRegenGainView {
    pub text: String,
    pub mark: Option<&'static str>,
    pub condition: Option<&'static str>,
}

impl ddd::Layered for MutedHitPointsRegenGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MutedHitPointsRegenGainView {}
