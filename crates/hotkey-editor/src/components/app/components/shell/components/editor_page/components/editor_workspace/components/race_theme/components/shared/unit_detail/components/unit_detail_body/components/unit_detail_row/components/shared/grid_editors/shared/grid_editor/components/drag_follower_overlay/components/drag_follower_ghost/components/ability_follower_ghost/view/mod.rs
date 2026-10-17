use super::super::super::presentation::FollowerPresentation;

#[derive(Clone, PartialEq)]
pub struct AbilityFollowerGhostView {
    pub presentation: FollowerPresentation,
}

impl ddd::Layered for AbilityFollowerGhostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AbilityFollowerGhostView {}
