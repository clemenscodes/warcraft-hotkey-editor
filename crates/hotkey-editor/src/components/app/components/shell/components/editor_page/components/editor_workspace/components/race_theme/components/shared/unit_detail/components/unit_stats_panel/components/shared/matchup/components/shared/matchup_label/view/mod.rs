use super::super::super::super::subject::MatchupSubject;

#[derive(Clone, PartialEq)]
pub struct MatchupLabelView {
    pub subject: MatchupSubject,
}

impl ddd::View for MatchupLabelView {}
