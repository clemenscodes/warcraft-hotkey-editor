use super::super::super::subject::MatchupSubject;
use super::view::WeakMatchupHostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WeakMatchupHostModel {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}

impl From<&WeakMatchupHostView> for WeakMatchupHostModel {
    fn from(view: &WeakMatchupHostView) -> Self {
        let WeakMatchupHostView {
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

impl ddd::Layered for WeakMatchupHostModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for WeakMatchupHostModel {
    type View = WeakMatchupHostView;
}
