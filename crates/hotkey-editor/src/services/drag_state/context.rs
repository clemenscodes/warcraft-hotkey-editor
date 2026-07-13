use dioxus::prelude::*;

use crate::services::drag_state::{DragFollower, DragState, DraggingSlot, DropTargetTile};

/// Access the [`DragState`] provided by the app shell. Call from a component or hook
/// body (it is a hook). The in-progress drag is a shell-wide concept the grid editor
/// and the app Escape handler read from context, so this accessor lives beside the type
/// in `services/`, not colocated with any component.
pub(crate) fn use_drag_state() -> DragState {
    use_context()
}

/// Create the three drag signals (all start empty — a drag is only ever in progress at
/// runtime, never seeded from the URL), assemble the [`DragState`], provide it as
/// context, and hand it back. The shell calls this once on boot; the grid editor and
/// the Escape handler read the result through [`use_drag_state`].
pub(crate) fn use_drag_state_provider() -> DragState {
    let dragging_slot = use_signal::<Option<DraggingSlot>>(|| None);
    let drop_target_tile = use_signal::<Option<DropTargetTile>>(|| None);
    let drag_follower = use_signal::<Option<DragFollower>>(|| None);
    let drag_state = DragState::new(dragging_slot, drop_target_tile, drag_follower);
    use_context_provider(|| drag_state);
    drag_state
}
