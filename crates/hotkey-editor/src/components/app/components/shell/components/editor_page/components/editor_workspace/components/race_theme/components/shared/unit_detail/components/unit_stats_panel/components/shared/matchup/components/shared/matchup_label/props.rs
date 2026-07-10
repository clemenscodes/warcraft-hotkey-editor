use super::super::super::super::subject::MatchupSubject;
use super::view::MatchupLabelView;
use dioxus::prelude::*;

/// A matchup cell's subject: the attack or defense type it names, rendered through the
/// subject's own `Display`.
#[derive(Props, Clone, PartialEq)]
pub struct MatchupLabelProps {
    pub subject: MatchupSubject,
}

impl From<&MatchupLabelView> for MatchupLabelProps {
    fn from(view: &MatchupLabelView) -> Self {
        let MatchupLabelView { subject } = view.clone();
        Self { subject }
    }
}

impl ddd::Props for MatchupLabelProps {
    type View = MatchupLabelView;
}
