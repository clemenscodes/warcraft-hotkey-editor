use super::super::super::presentation::FollowerPresentation;

#[derive(Clone, PartialEq)]
pub struct AbilityFollowerGhostView {
    pub presentation: FollowerPresentation,
}

impl ddd::View for AbilityFollowerGhostView {}
