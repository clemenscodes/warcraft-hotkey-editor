use crate::services::editor_state::DragFollower;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DragFollowerOverlayProps {
    pub drag_follower: Signal<Option<DragFollower>>,
    /// Whether this grid owns the in-progress drag. The stylesheet is always
    /// emitted (so it is in `<head>` before any drag, avoiding a first-paint
    /// flicker), but the follower element only renders when visible.
    #[props(default)]
    pub visible: bool,
}
