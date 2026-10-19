use super::super::super::subject::MatchupSubject;

#[derive(Clone, PartialEq)]
pub struct StrongMatchupHostView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
}

impl ddd::Layered for StrongMatchupHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for StrongMatchupHostView {}
