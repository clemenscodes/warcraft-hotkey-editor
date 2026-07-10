use super::super::super::logic::FollowerPresentation;

/// The published `View` contract mirroring [`AbilityFollowerGhostProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AbilityFollowerGhostView {
    pub presentation: FollowerPresentation,
}

impl ddd::View for AbilityFollowerGhostView {}
