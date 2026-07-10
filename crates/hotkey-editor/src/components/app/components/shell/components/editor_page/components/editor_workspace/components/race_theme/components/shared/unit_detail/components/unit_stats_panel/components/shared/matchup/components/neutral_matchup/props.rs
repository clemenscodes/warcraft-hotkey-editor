use super::super::super::subject::MatchupSubject;
use dioxus::prelude::*;

/// The neutral matchup cell: no tint.
#[derive(Props, Clone, PartialEq)]
pub struct NeutralMatchupProps {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}
