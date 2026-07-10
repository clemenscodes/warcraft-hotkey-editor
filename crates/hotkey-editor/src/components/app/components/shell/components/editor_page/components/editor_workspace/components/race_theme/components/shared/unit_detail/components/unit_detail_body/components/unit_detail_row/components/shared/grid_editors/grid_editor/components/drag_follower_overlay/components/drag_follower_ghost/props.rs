use super::logic::FollowerPresentation;
use super::view::DragFollowerGhostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DragFollowerGhostProps {
    /// The follower's presentation when this grid owns the in-progress drag, or
    /// `None` when there is nothing to show.
    pub presentation: Option<FollowerPresentation>,
}

impl From<&DragFollowerGhostView> for DragFollowerGhostProps {
    fn from(view: &DragFollowerGhostView) -> Self {
        let DragFollowerGhostView { presentation } = view.clone();
        Self { presentation }
    }
}

impl ddd::Props for DragFollowerGhostProps {
    type View = DragFollowerGhostView;
}
