use std::ops::Range;

use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

use crate::components::command_grid::{GridTileState, HotkeyBadgeState};
use crate::model::grid::DragFollowerVisual;

use super::super::props::CommandGridProps;
use super::mechanics;

/// One tile, fully prepared for rendering: every value and event handler the
/// presentational `GridTile` needs. The component file builds none of this; it
/// destructures each prepared tile and renders.
pub(crate) struct PreparedTile {
    pub(crate) column: u8,
    pub(crate) row: u8,
    pub(crate) icon: Option<String>,
    pub(crate) label: String,
    pub(crate) hotkey: Option<String>,
    pub(crate) badge_state: HotkeyBadgeState,
    pub(crate) state: GridTileState,
    pub(crate) is_dragging_source: bool,
    pub(crate) is_drag_over: bool,
    pub(crate) is_focusable: bool,
    pub(crate) draggable: bool,
    pub(crate) onkeydown: EventHandler<KeyboardEvent>,
    pub(crate) onpointerdown: EventHandler<PointerEvent>,
    pub(crate) onpointermove: EventHandler<PointerEvent>,
    pub(crate) onpointerup: EventHandler<PointerEvent>,
    pub(crate) onpointercancel: EventHandler<PointerEvent>,
    pub(crate) onlostpointercapture: EventHandler<PointerEvent>,
    pub(crate) onclick: EventHandler<MouseEvent>,
    pub(crate) ondoubleclick: EventHandler<MouseEvent>,
}

/// The whole grid prepared for rendering: the tiles and whether the in-progress
/// drag started in this grid (so the follower shows here).
pub(crate) struct GridRender {
    pub(crate) tiles: Vec<PreparedTile>,
    pub(crate) drag_active_here: bool,
}

impl GridRender {
    /// Builds every tile's visual state and event handlers, overlaying drag states
    /// onto empty tiles via the `drop_blocked` callback. This is all the grid's
    /// per-tile logic, kept out of the markup.
    pub(crate) fn new(props: &CommandGridProps) -> Self {
        let grid_id = props.grid_id;
        let dragging_slot = props.dragging_slot;
        let drop_target_tile = props.drop_target_tile;
        let drag_follower = props.drag_follower;
        let on_select = props.on_select;
        let on_activate = props.on_activate;
        let on_move = props.on_move;
        let drop_blocked = props.drop_blocked;

        let dragging_value = *dragging_slot.read();
        let drop_target_value = *drop_target_tile.read();
        let dragging_source_coordinate = dragging_value
            .filter(|detail| detail.grid_id() == grid_id)
            .map(|detail| detail.coordinate());
        let drag_active_here = dragging_source_coordinate.is_some();

        let mut tiles = Vec::with_capacity(props.views.len());
        for view in props.views.iter() {
            let coordinate = view.coordinate();
            let column = view.column();
            let row = view.row();
            let base_state = view.state();
            let has_occupant = base_state != GridTileState::Empty;
            let is_dragging_source = dragging_source_coordinate == Some(coordinate);
            let is_drag_over = drag_active_here
                && drop_target_value
                    .map(|target| target.grid_id() == grid_id && target.coordinate() == coordinate)
                    .unwrap_or(false);
            let tile_state = tile_state(
                base_state,
                coordinate,
                dragging_source_coordinate,
                drop_blocked,
            );

            let icon = view.icon().map(|source| source.to_string());
            let label = view.label().to_string();
            let hotkey = view.hotkey().map(|letter| letter.to_string());
            let badge_state = view.badge_state();
            let draggable = view.draggable();

            let follower_icon = view.icon().map(|source| source.to_string());
            let follower_label = view.label().to_string();
            let follower_letter = view.hotkey().map(|letter| letter.to_string());
            let visual = DragFollowerVisual::new(
                follower_icon,
                follower_label,
                follower_letter,
                view.is_passive(),
                view.is_command(),
            );

            let keydown = mechanics::keydown(on_select, coordinate);
            let pointer_down_args = mechanics::PointerDownArgs {
                draggable,
                dragging_slot,
                drop_target_tile,
                drag_follower,
                visual,
                grid_id,
                coordinate,
            };
            let pointer_down = mechanics::pointer_down(pointer_down_args);
            let pointer_move =
                mechanics::pointer_move(dragging_slot, drop_target_tile, drag_follower, grid_id);
            let pointer_up_args = mechanics::PointerUpArgs {
                dragging_slot,
                drop_target_tile,
                drag_follower,
                on_move,
                on_select,
                grid_id,
            };
            let pointer_up = mechanics::pointer_up(pointer_up_args);
            let pointer_cancel =
                mechanics::pointer_cancel(dragging_slot, drop_target_tile, drag_follower);
            let lost_pointer_capture =
                mechanics::lost_pointer_capture(dragging_slot, drop_target_tile, drag_follower);
            let click = mechanics::click(on_select, coordinate);
            let double_click = mechanics::double_click(on_activate, coordinate);

            let prepared = PreparedTile {
                column,
                row,
                icon,
                label,
                hotkey,
                badge_state,
                state: tile_state,
                is_dragging_source,
                is_drag_over,
                is_focusable: has_occupant,
                draggable,
                onkeydown: EventHandler::new(keydown),
                onpointerdown: EventHandler::new(pointer_down),
                onpointermove: EventHandler::new(pointer_move),
                onpointerup: EventHandler::new(pointer_up),
                onpointercancel: EventHandler::new(pointer_cancel),
                onlostpointercapture: EventHandler::new(lost_pointer_capture),
                onclick: EventHandler::new(click),
                ondoubleclick: EventHandler::new(double_click),
            };
            tiles.push(prepared);
        }

        Self {
            tiles,
            drag_active_here,
        }
    }
}

/// The final tile state: an occupied tile keeps its base state; an empty tile
/// during a drag becomes a drop target, or a blocked one when the callback
/// refuses the move.
fn tile_state(
    base_state: GridTileState,
    coordinate: GridCoordinate,
    dragging_source_coordinate: Option<GridCoordinate>,
    drop_blocked: Callback<Range<GridCoordinate>, bool>,
) -> GridTileState {
    if base_state != GridTileState::Empty {
        return base_state;
    }
    let Some(source_coordinate) = dragging_source_coordinate else {
        return GridTileState::Empty;
    };
    let attempted_move = source_coordinate..coordinate;
    if drop_blocked.call(attempted_move) {
        GridTileState::BlockedDropTarget
    } else {
        GridTileState::DropTarget
    }
}
