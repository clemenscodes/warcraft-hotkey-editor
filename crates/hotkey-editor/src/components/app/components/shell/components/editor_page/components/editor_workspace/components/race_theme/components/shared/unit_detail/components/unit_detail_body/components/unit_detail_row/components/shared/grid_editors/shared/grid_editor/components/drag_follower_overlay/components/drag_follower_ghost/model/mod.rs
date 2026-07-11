use super::presentation::FollowerPresentation;
use super::view::DragFollowerGhostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DragFollowerGhostModel {
    /// The follower's presentation when this grid owns the in-progress drag, or
    /// `None` when there is nothing to show.
    pub presentation: Option<FollowerPresentation>,
}

impl From<&DragFollowerGhostView> for DragFollowerGhostModel {
    fn from(view: &DragFollowerGhostView) -> Self {
        let DragFollowerGhostView { presentation } = view.clone();
        Self { presentation }
    }
}

impl ddd::Model for DragFollowerGhostModel {
    type View = DragFollowerGhostView;
}
