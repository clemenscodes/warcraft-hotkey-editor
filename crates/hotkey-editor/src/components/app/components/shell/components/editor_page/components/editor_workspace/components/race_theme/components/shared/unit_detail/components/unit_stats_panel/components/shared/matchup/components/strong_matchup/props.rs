use super::super::super::subject::MatchupSubject;
use dioxus::prelude::*;

/// The strong matchup cell: a success-green tint.
#[derive(Props, Clone, PartialEq)]
pub struct StrongMatchupProps {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}
