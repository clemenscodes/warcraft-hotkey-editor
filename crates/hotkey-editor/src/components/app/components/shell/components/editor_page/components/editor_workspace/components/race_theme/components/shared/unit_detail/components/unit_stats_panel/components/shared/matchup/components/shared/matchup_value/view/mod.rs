#[derive(Clone, PartialEq)]
pub struct MatchupValueView {
    pub multiplier: f32,
}

impl ddd::Layered for MatchupValueView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MatchupValueView {}
