use super::super::super::subject::MatchupSubject;
use dioxus::prelude::*;

/// The weak matchup cell: a danger tint.
#[derive(Props, Clone, PartialEq)]
pub struct WeakMatchupProps {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}
