use super::components::drag_follower_ghost::FollowerPresentation;
use super::props::DragFollowerOverlayProps;
use dioxus::prelude::*;

/// Shapes the follower ghost's presentation: the dragged tile's captured visual when this
/// grid owns the in-progress drag, or nothing when it does not. The drag state is a UI
/// signal, so reading it and deriving the presentation is the overlay's seam — the ghost
/// leaf it feeds stays a pure function of the shaped value.
pub(super) fn use_drag_follower_overlay(
    props: &DragFollowerOverlayProps,
) -> Option<FollowerPresentation> {
    let active = if props.visible {
        props.drag_follower.read().clone()
    } else {
        None
    };
    active.as_ref().map(FollowerPresentation::from)
}
