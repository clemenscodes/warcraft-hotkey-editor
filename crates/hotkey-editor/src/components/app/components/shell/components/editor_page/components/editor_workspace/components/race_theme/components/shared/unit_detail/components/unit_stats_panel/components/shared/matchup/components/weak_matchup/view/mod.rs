use super::super::super::subject::MatchupSubject;

#[derive(Clone, PartialEq)]
pub struct WeakMatchupView {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    pub title: String,
}

impl ddd::View for WeakMatchupView {}
