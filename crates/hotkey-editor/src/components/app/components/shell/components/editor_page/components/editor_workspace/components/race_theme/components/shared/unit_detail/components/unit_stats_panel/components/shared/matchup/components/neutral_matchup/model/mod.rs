use super::super::super::subject::MatchupSubject;
use super::view::NeutralMatchupView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NeutralMatchupModel {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}

impl From<&NeutralMatchupView> for NeutralMatchupModel {
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

impl ddd::Model for NeutralMatchupModel {
    type View = NeutralMatchupView;
}
