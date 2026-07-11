use super::super::super::subject::MatchupSubject;

/// The published `View` contract mirroring [`StrongMatchupModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct StrongMatchupView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
}

impl ddd::View for StrongMatchupView {}
