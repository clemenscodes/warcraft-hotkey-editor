use super::super::super::subject::MatchupSubject;
use super::view::NeutralMatchupView;
use dioxus::prelude::*;

/// The neutral matchup cell: no tint.
#[derive(Props, Clone, PartialEq)]
pub struct NeutralMatchupProps {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}

impl From<&NeutralMatchupView> for NeutralMatchupProps {
    fn from(view: &NeutralMatchupView) -> Self {
        let NeutralMatchupView {
            subject,
            multiplier,
            title,
        } = view.clone();
        Self {
            subject,
            multiplier,
            title,
        }
    }
}

impl ddd::Props for NeutralMatchupProps {
    type View = NeutralMatchupView;
}
