use super::super::super::subject::MatchupSubject;
use super::view::StrongMatchupHostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StrongMatchupHostModel {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}

impl From<&StrongMatchupHostView> for StrongMatchupHostModel {
    fn from(view: &StrongMatchupHostView) -> Self {
        let StrongMatchupHostView {
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

impl ddd::Layered for StrongMatchupHostModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for StrongMatchupHostModel {
    type View = StrongMatchupHostView;
}
