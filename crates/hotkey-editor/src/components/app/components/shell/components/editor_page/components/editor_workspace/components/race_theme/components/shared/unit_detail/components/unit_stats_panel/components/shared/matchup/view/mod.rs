use super::state::MatchupStrength;
use super::subject::MatchupSubject;

/// The published `View` contract mirroring [`MatchupModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MatchupView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
    pub strength: MatchupStrength,
}

impl ddd::View for MatchupView {}
