use super::super::super::super::subject::MatchupSubject;

#[derive(Clone, PartialEq)]
pub struct MatchupLabelView {
    pub subject: MatchupSubject,
}

impl ddd::Layered for MatchupLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MatchupLabelView {}
