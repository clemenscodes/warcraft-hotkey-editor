use super::components::drag_follower_ghost::FollowerPresentation;
use super::model::DragFollowerOverlayModel;
use dioxus::prelude::*;

pub(super) fn use_drag_follower_overlay(
    props: &DragFollowerOverlayModel,
) -> Option<FollowerPresentation> {
    let active = if props.visible {
        props.drag_follower.read().clone()
    } else {
        None
    };
    active.as_ref().map(FollowerPresentation::from)
}
