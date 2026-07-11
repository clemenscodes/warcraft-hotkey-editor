use super::super::super::presentation::FollowerPresentation;

/// The published `View` contract mirroring [`CommandFollowerGhostModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CommandFollowerGhostView {
    pub presentation: FollowerPresentation,
}

impl ddd::View for CommandFollowerGhostView {}
