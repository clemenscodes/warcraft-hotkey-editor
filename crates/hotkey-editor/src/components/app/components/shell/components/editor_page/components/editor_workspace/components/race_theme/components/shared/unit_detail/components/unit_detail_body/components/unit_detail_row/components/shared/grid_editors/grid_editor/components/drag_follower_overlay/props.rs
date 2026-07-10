use super::view::DragFollowerOverlayView;
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

impl From<&DragFollowerOverlayView> for DragFollowerOverlayProps {
    fn from(view: &DragFollowerOverlayView) -> Self {
        let DragFollowerOverlayView {
            drag_follower,
            visible,
        } = view.clone();
        Self {
            drag_follower,
            visible,
        }
    }
}

impl ddd::Props for DragFollowerOverlayProps {
    type View = DragFollowerOverlayView;
}
