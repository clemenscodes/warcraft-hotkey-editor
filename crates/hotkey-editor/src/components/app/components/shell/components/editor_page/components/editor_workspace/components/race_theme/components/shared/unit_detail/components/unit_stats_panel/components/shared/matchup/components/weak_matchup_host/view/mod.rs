use super::super::super::subject::MatchupSubject;

#[derive(Clone, PartialEq)]
pub struct WeakMatchupHostView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
}

impl ddd::Layered for WeakMatchupHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for WeakMatchupHostView {}
