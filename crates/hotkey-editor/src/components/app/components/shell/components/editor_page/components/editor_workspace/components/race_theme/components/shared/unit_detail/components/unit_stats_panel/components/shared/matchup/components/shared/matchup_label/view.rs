use super::super::super::super::subject::MatchupSubject;

/// The published `View` contract mirroring [`MatchupLabelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MatchupLabelView {
    pub subject: MatchupSubject,
}

impl ddd::View for MatchupLabelView {}
