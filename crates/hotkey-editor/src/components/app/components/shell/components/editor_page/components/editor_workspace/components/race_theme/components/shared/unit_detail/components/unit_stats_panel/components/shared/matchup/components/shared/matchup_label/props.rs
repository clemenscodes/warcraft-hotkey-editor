use super::super::super::super::subject::MatchupSubject;
use dioxus::prelude::*;

/// A matchup cell's subject: the attack or defense type it names, rendered through the
/// subject's own `Display`.
#[derive(Props, Clone, PartialEq)]
pub struct MatchupLabelProps {
    pub subject: MatchupSubject,
}
