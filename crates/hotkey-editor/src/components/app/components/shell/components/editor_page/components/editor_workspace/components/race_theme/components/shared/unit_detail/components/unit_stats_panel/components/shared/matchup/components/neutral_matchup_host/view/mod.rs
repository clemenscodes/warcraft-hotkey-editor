use super::super::super::subject::MatchupSubject;

#[derive(Clone, PartialEq)]
pub struct NeutralMatchupHostView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
}

impl ddd::Layered for NeutralMatchupHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for NeutralMatchupHostView {}
