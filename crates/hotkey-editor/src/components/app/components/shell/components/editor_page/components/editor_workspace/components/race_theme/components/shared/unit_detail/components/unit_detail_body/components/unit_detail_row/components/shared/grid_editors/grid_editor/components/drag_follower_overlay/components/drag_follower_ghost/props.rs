use super::logic::FollowerPresentation;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DragFollowerGhostProps {
    /// The follower's presentation when this grid owns the in-progress drag, or
    /// `None` when there is nothing to show.
    pub presentation: Option<FollowerPresentation>,
}
