use super::super::super::subject::MatchupSubject;
use super::view::WeakMatchupView;
use dioxus::prelude::*;

/// The weak matchup cell: a danger tint.
#[derive(Props, Clone, PartialEq)]
pub struct WeakMatchupProps {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}

impl From<&WeakMatchupView> for WeakMatchupProps {
    fn from(view: &WeakMatchupView) -> Self {
        let WeakMatchupView {
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

impl ddd::Props for WeakMatchupProps {
    type View = WeakMatchupView;
}
