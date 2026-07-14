use super::super::super::subject::MatchupSubject;
use super::view::StrongMatchupView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StrongMatchupModel {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}

impl From<&StrongMatchupView> for StrongMatchupModel {
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

impl ddd::Model for StrongMatchupModel {
    type View = StrongMatchupView;
}
