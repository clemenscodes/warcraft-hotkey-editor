use super::super::super::subject::MatchupSubject;

#[derive(Clone, PartialEq)]
pub struct StrongMatchupView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
}

impl ddd::Layered for StrongMatchupView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for StrongMatchupView {}
