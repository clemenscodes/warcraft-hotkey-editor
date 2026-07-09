use super::super::super::DragFollowerOverlayProps;
use super::logic::FollowerPresentation;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DragFollowerGhostProps {
    /// The follower's presentation when this grid owns the in-progress drag, or
    /// `None` when there is nothing to show.
    pub presentation: Option<FollowerPresentation>,
}

impl From<&DragFollowerOverlayProps> for DragFollowerGhostProps {
    /// The ghost shows, when this grid owns the drag, the dragged tile's presentation;
    /// its accent colour is read from the inherited `--race-accent`.
    fn from(props: &DragFollowerOverlayProps) -> Self {
        let active = if props.visible {
            props.drag_follower.read().clone()
        } else {
            None
        };
        let presentation = active.as_ref().map(FollowerPresentation::from);
        Self { presentation }
    }
}
