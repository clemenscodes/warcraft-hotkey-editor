use dioxus::prelude::*;

use crate::services::drag_state::{DragFollower, DragState, DraggingSlot, DropTargetTile};

pub(crate) fn use_drag_state() -> DragState {
    use_context()
}

pub(crate) fn use_drag_state_provider() -> DragState {
    let dragging_slot = use_signal::<Option<DraggingSlot>>(|| None);
    let drop_target_tile = use_signal::<Option<DropTargetTile>>(|| None);
    let drag_follower = use_signal::<Option<DragFollower>>(|| None);
    let drag_state = DragState::new(dragging_slot, drop_target_tile, drag_follower);
    use_context_provider(|| drag_state);
    drag_state
}
