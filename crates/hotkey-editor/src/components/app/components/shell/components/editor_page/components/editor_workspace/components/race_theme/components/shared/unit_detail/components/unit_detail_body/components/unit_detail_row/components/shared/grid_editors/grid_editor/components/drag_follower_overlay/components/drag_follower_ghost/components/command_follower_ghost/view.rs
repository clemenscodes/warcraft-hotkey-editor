use super::super::super::logic::FollowerPresentation;

/// The published `View` contract mirroring [`CommandFollowerGhostProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CommandFollowerGhostView {
    pub presentation: FollowerPresentation,
}

impl ddd::View for CommandFollowerGhostView {}
