use dioxus::prelude::*;

pub mod context;
pub mod drag;
pub mod hit_test;

pub use drag::{DragFollower, DragFollowerVisual, DraggingSlot, DropTargetTile};
pub(crate) use hit_test::{CursorPoint, HitTestPoint};

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
