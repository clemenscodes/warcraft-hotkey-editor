use std::ops::Range;

use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

use crate::components::command_grid::GridTileProps;
use crate::components::command_grid::GridTileState;
use crate::model::grid::DragFollowerVisual;

use super::super::props::CommandGridProps;
use super::mechanics;

/// The whole grid prepared for rendering: one finished `GridTileProps` per tile.
/// Whether the follower shows here is derived separately by
/// `DragFollowerOverlayProps::from`.
pub(crate) struct GridRender {
    pub(crate) tiles: Vec<GridTileProps>,
}

impl From<&CommandGridProps> for GridRender {
    /// Builds every tile's props and event handlers, overlaying drag states onto
    /// empty tiles via the `drop_blocked` callback. This is all the grid's per-tile
    /// logic, kept out of the markup.
    fn from(props: &CommandGridProps) -> Self {
        let grid_id = props.grid_id;
        let race = props.race;
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
            let hotkey = view.hotkey();
            let badge_state = view.badge_state();
            let draggable = view.draggable();

            // A follower is only ever shown for a draggable tile, which always has
            // an icon, so the visual exists exactly when the icon does.
            let visual = view.icon().map(|source| {
                let icon_source = source.to_string();
                let label_text = view.label().to_string();
                let displayed_letter = view.hotkey();
                let is_passive = view.is_passive();
                let is_command = view.is_command();
                DragFollowerVisual::new(
                    icon_source,
                    label_text,
                    displayed_letter,
                    is_passive,
                    is_command,
                )
            });

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

            let tile = GridTileProps {
                coordinate,
                race,
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
            tiles.push(tile);
        }

        Self { tiles }
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
