use super::state::MatchupStrength;
use super::subject::MatchupSubject;
use super::view::MatchupView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MatchupModel {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
    #[props(default)]
    pub strength: MatchupStrength,
}

impl From<&MatchupView> for MatchupModel {
    fn from(view: &MatchupView) -> Self {
        let MatchupView {
            subject,
            multiplier,
            title,
            strength,
        } = view.clone();
        Self {
            subject,
            multiplier,
            title,
            strength,
        }
    }
}

impl ddd::Model for MatchupModel {
    type View = MatchupView;
}
