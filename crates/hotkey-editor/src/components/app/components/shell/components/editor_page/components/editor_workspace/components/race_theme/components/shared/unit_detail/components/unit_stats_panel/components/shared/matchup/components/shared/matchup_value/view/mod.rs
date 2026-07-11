/// The published `View` contract mirroring [`MatchupValueModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MatchupValueView {
    pub multiplier: f32,
}

impl ddd::View for MatchupValueView {}
