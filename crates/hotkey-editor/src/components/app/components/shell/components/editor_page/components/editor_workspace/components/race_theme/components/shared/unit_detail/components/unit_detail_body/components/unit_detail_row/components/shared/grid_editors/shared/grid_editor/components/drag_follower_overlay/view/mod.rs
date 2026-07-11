use crate::services::editor_state::DragFollower;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`DragFollowerOverlayModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DragFollowerOverlayView {
    pub drag_follower: Signal<Option<DragFollower>>,
    /// Whether this grid owns the in-progress drag. The stylesheet is always
    /// emitted (so it is in `<head>` before any drag, avoiding a first-paint
    /// flicker), but the follower element only renders when visible.
    pub visible: bool,
}

impl ddd::View for DragFollowerOverlayView {}
