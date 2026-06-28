use std::ops::Range;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_keybinds::GridCoordinate;

use crate::model::grid::{DragFollower, DraggingSlot, DropTargetTile};

use super::view::GridTileView;

/// The generic command-grid engine's inputs. It renders finished `GridTileView`s,
/// owns the drag mechanics, and reports gestures through plain callbacks. It
/// carries no domain type: it never knows which ability a cell holds, what a move
/// means, or why a drop is blocked, nor that a heading exists. The `grid_id` is an
/// opaque identifier used only to scope drag hit-testing between sibling grids and
/// as the `data-grid-id` attribute.
#[derive(Props, Clone, PartialEq)]
pub struct CommandGridProps {
    pub views: Rc<[GridTileView]>,
    pub grid_id: &'static str,
    #[props(default = Race::Neutral)]
    pub race: Race,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub on_select: EventHandler<GridCoordinate>,
    pub on_activate: EventHandler<GridCoordinate>,
    pub on_move: EventHandler<Range<GridCoordinate>>,
    /// Asked, for an empty cell during an active drag, whether a drop there must
    /// be painted as blocked. The grid does not know why a move is blocked.
    pub drop_blocked: Callback<Range<GridCoordinate>, bool>,
}
