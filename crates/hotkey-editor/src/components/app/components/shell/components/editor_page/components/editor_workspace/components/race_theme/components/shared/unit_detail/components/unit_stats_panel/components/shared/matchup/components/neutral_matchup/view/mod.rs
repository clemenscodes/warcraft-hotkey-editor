use super::super::super::subject::MatchupSubject;

/// The published `View` contract mirroring [`NeutralMatchupModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct NeutralMatchupView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
}

impl ddd::View for NeutralMatchupView {}
