#[derive(Clone, PartialEq)]
pub struct ActiveHitPointsRegenGainView {
    pub text: String,
    pub condition: Option<&'static str>,
}

impl ddd::Layered for ActiveHitPointsRegenGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveHitPointsRegenGainView {}
