use super::super::super::super::super::subject::MatchupSubject;

#[derive(Clone, PartialEq)]
pub struct NeutralMatchupView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
}

impl ddd::Layered for NeutralMatchupView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for NeutralMatchupView {}
