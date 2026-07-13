use dioxus::prelude::*;

pub mod context;
pub mod drag;
pub mod hit_test;

pub use drag::{DragFollower, DragFollowerVisual, DraggingSlot, DropTargetTile};
pub(crate) use hit_test::{CursorPoint, HitTestPoint};

/// The in-progress grid-editor drag: the source cell being dragged, the tile currently
/// hovered as a drop target, and the floating follower rendered under the cursor. This
/// is the grid editor's own concern — only the grid editor and the app-level Escape
/// handler touch it — split out of the broader [`EditorState`] rather than lumped into
/// it. Provided once at the app root and read with `use_context`. Each field is a
/// `Signal`, so a reader subscribes only to the slice it touches; the whole struct is
/// `Copy`, so a handler captures it cheaply.
///
/// [`EditorState`]: crate::services::editor_state::EditorState
#[derive(Clone, Copy, PartialEq)]
pub struct DragState {
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_tile: Signal<Option<DropTargetTile>>,
    drag_follower: Signal<Option<DragFollower>>,
}

impl DragState {
    pub fn new(
        dragging_slot: Signal<Option<DraggingSlot>>,
        drop_target_tile: Signal<Option<DropTargetTile>>,
        drag_follower: Signal<Option<DragFollower>>,
    ) -> Self {
        Self {
            dragging_slot,
            drop_target_tile,
            drag_follower,
        }
    }

    pub fn dragging_slot(&self) -> Signal<Option<DraggingSlot>> {
        self.dragging_slot
    }

    pub fn drop_target_tile(&self) -> Signal<Option<DropTargetTile>> {
        self.drop_target_tile
    }

    pub fn drag_follower(&self) -> Signal<Option<DragFollower>> {
        self.drag_follower
    }
}
