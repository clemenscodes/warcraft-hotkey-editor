use super::super::super::presentation::FollowerPresentation;

/// The published `View` contract mirroring [`AbilityFollowerGhostModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AbilityFollowerGhostView {
    pub presentation: FollowerPresentation,
}

impl ddd::View for AbilityFollowerGhostView {}
