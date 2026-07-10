use super::super::super::subject::MatchupSubject;
use super::view::StrongMatchupView;
use dioxus::prelude::*;

/// The strong matchup cell: a success-green tint.
#[derive(Props, Clone, PartialEq)]
pub struct StrongMatchupProps {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}

impl From<&StrongMatchupView> for StrongMatchupProps {
    fn from(view: &StrongMatchupView) -> Self {
        let StrongMatchupView {
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

impl ddd::Props for StrongMatchupProps {
    type View = StrongMatchupView;
}
