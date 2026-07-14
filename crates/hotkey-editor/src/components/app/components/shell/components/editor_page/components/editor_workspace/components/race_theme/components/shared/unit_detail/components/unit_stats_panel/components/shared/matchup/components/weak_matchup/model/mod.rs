use super::super::super::subject::MatchupSubject;
use super::view::WeakMatchupView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WeakMatchupModel {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
}

impl From<&WeakMatchupView> for WeakMatchupModel {
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

impl ddd::Model for WeakMatchupModel {
    type View = WeakMatchupView;
}
