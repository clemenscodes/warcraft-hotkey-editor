use crate::components::app::components::shell::components::toasts::consume_toast;
use crate::services::customkeys::service::CustomKeysService;
use dioxus::prelude::*;
use std::ops::Range;

use warcraft_keybinds::{
    COMMAND_GRID_TILE_COUNT, ColumnIndex, GridBehavior, GridCoordinate, HotkeyToken, RenderedTile,
    RowIndex,
};

use crate::components::app::components::shell::components::shared::grid_tile::GridTileState;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::editor_headed_grid::components::editor_grid::components::grid_editor_tile::EditorTile;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::HotkeyBadgeState;

use super::super::props::GridEditorProps;
use crate::services::editor_state::DragFollowerVisual;

use super::handlers::{
    MoveHandlerArgs, activate_handler, drop_blocked_callback, move_handler, select_handler,
};

use super::mechanics;

/// Builds the editor's finished tiles. `rendered_tiles` is already resolved by the
/// caller (`GridEditor`, in a `use_memo` in its own reactive scope) — that is what lets
/// one grid's edit avoid re-rendering its siblings, so this builder must not itself read
/// `loaded_keys` or any other grid-state signal. It only adapts each rendered tile into
/// an `EditorTile` and overlays the drag state plus every pointer handler on top. This
/// is all the grid's behavior, kept in the editor; the `EditorHeadedGrid` and the
/// `EditorGrid` only draw what comes out. Always exactly `COMMAND_GRID_TILE_COUNT` tiles,
/// so the result is a fixed-size array.
impl EditorTile {
    pub(crate) fn grid<B: GridBehavior>(
        props: &GridEditorProps<B>,
        rendered_tiles: Vec<RenderedTile>,
    ) -> [Self; COMMAND_GRID_TILE_COUNT] {
        let behavior = props.behavior.clone();
        let config = &props.config;
        let toast = consume_toast();
        let grid_id = config.heading;
        let loaded_keys = config.loaded_keys;
        let grid_layout = config.grid_layout;
        let selected_slot = config.selected_slot;
        let selected_from_research = config.selected_from_research;
        let selected_from_uprooted = config.selected_from_uprooted;
        let update_hotkeys_on_move = config.update_hotkeys_on_move;
        let hotkey_assign_request = config.hotkey_assign_request;
        let prevent_swap_on_drop = config.prevent_swap_on_drop;
        let slot_ids = config.slot_ids.clone();
        let dragging_slot = config.dragging_slot;
        let drop_target_tile = config.drop_target_tile;
        let drag_follower = config.drag_follower;
        let on_select = select_handler(
            behavior.clone(),
            loaded_keys,
            selected_slot,
            selected_from_research,
            selected_from_uprooted,
            slot_ids.clone(),
        );
        let on_activate = activate_handler(
            behavior.clone(),
            loaded_keys,
            selected_slot,
            selected_from_research,
            selected_from_uprooted,
            hotkey_assign_request,
            slot_ids.clone(),
        );
        let move_args = MoveHandlerArgs {
            behavior: behavior.clone(),
            loaded_keys,
            custom_keys_service: consume_context::<CustomKeysService>(),
            grid_layout,
            selected_slot,
            update_hotkeys_on_move,
            prevent_swap_on_drop,
            slot_ids: slot_ids.clone(),
            toast,
        };
        let on_move = move_handler(move_args);
        let drop_blocked = drop_blocked_callback(behavior.clone(), loaded_keys, slot_ids.clone());
        let dragging_value = *dragging_slot.read();
        let drop_target_value = *drop_target_tile.read();
        let dragging_source_coordinate = dragging_value
            .filter(|detail| detail.grid_id() == grid_id)
            .map(|detail| detail.coordinate());
        let drag_active_here = dragging_source_coordinate.is_some();
        let mut tile_props_list: Vec<Self> = Vec::with_capacity(rendered_tiles.len());
        for rendered in rendered_tiles.iter() {
            let mut tile = Self::from(rendered);
            let coordinate = tile.coordinate;
            let base_state = tile.state;
            let draggable = tile.draggable;
            let is_dragging_source = dragging_source_coordinate == Some(coordinate);
            let is_drag_over = drag_active_here
                && drop_target_value
                    .map(|target| target.grid_id() == grid_id && target.coordinate() == coordinate)
                    .unwrap_or(false);
            let final_state = tile_state(
                base_state,
                coordinate,
                dragging_source_coordinate,
                drop_blocked,
            );
            let visual = tile.icon.clone().map(|icon_source| {
                let label_text = tile.label.clone();
                let displayed_letter = tile.hotkey;
                let is_passive = rendered.is_passive();
                let is_command = rendered.is_command();
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
            tile.is_dragging_source = is_dragging_source;
            tile.is_drag_over = is_drag_over;
            tile.state = final_state;
            tile.onkeydown = EventHandler::new(keydown);
            tile.onpointerdown = EventHandler::new(pointer_down);
            tile.onpointermove = EventHandler::new(pointer_move);
            tile.onpointerup = EventHandler::new(pointer_up);
            tile.onpointercancel = EventHandler::new(pointer_cancel);
            tile.onlostpointercapture = EventHandler::new(lost_pointer_capture);
            tile.onclick = EventHandler::new(click);
            tile.ondoubleclick = EventHandler::new(double_click);
            tile_props_list.push(tile);
        }
        let tiles: [Self; COMMAND_GRID_TILE_COUNT] = tile_props_list
            .try_into()
            .unwrap_or_else(|_| std::array::from_fn(|_| placeholder_tile()));
        tiles
    }
}

/// An inert, empty tile used only as a fallback when the memoized rendered
/// tiles are transiently not exactly `COMMAND_GRID_TILE_COUNT` long (e.g. the
/// first frame before boot has resolved `loaded_keys`). Renders an empty,
/// non-interactive square instead of panicking; the installed panic hook
/// still surfaces the real cause if this path is ever hit outside boot.
fn placeholder_tile() -> EditorTile {
    let coordinate = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
    EditorTile {
        coordinate,
        icon: None,
        label: String::new(),
        hotkey: HotkeyToken::Escape,
        badge_state: HotkeyBadgeState::default(),
        state: GridTileState::default(),
        is_dragging_source: false,
        is_drag_over: false,
        is_focusable: false,
        draggable: false,
        onkeydown: EventHandler::default(),
        onpointerdown: EventHandler::default(),
        onpointermove: EventHandler::default(),
        onpointerup: EventHandler::default(),
        onpointercancel: EventHandler::default(),
        onlostpointercapture: EventHandler::default(),
        onclick: EventHandler::default(),
        ondoubleclick: EventHandler::default(),
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
