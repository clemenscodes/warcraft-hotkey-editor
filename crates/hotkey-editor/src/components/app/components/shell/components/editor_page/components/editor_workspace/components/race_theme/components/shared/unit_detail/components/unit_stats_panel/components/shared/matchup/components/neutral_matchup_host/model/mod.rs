use super::super::super::subject::MatchupSubject;
use super::view::NeutralMatchupHostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NeutralMatchupHostModel {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}

impl From<&NeutralMatchupHostView> for NeutralMatchupHostModel {
    fn from(view: &NeutralMatchupHostView) -> Self {
        let NeutralMatchupHostView {
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

impl ddd::Layered for NeutralMatchupHostModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for NeutralMatchupHostModel {
    type View = NeutralMatchupHostView;
}
