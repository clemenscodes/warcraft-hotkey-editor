use super::state::MatchupStrength;
use super::subject::MatchupSubject;

#[derive(Clone, PartialEq)]
pub struct MatchupView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
    pub strength: MatchupStrength,
}

impl ddd::Layered for MatchupView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MatchupView {}
