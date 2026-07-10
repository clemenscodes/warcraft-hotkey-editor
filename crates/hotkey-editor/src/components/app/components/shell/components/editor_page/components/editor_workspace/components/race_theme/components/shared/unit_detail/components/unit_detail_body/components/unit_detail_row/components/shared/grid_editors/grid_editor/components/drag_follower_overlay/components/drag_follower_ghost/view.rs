use super::logic::FollowerPresentation;

/// The published `View` contract mirroring [`DragFollowerGhostProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DragFollowerGhostView {
    /// The follower's presentation when this grid owns the in-progress drag, or
    /// `None` when there is nothing to show.
    pub presentation: Option<FollowerPresentation>,
}

impl ddd::View for DragFollowerGhostView {}
