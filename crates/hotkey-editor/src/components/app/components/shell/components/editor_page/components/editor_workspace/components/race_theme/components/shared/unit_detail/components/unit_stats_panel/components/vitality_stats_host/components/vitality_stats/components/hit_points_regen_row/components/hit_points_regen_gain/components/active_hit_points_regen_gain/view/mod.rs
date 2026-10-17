#[derive(Clone, PartialEq)]
pub struct ActiveHitPointsRegenGainView {
    pub text: String,
}

impl ddd::Layered for ActiveHitPointsRegenGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveHitPointsRegenGainView {}
