use super::super::super::super::subject::MatchupSubject;
use super::view::MatchupLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MatchupLabelModel {
    pub subject: MatchupSubject,
}

impl From<&MatchupLabelView> for MatchupLabelModel {
    fn from(view: &MatchupLabelView) -> Self {
        let MatchupLabelView { subject } = view.clone();
        Self { subject }
    }
}

impl ddd::Model for MatchupLabelModel {
    type View = MatchupLabelView;
}
