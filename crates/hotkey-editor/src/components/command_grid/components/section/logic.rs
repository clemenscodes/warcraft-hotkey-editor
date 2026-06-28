use std::ops::Range;
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, Toasts, consume_toast};
use warcraft_keybinds::{
    CommandGridRenderInput, CustomKeys, GridBehavior, GridCoordinate, RenderedTile,
};

use crate::components::command_grid::components::{
    CommandGridHeadingProps, CommandGridProps, GridTileFlags, GridTileView,
};
use crate::components::command_grid::{GridTileState, HotkeyBadgeState};
use crate::model::icons::IconUrl;
use crate::services::customkeys::positions::{MoveRequest, Positions};
use crate::services::focus::modality::FocusModality;
use warcraft_keybinds::{GridLayout, GridSlotId};

use super::props::GridSectionProps;

impl<B: GridBehavior> From<&GridSectionProps<B>> for CommandGridHeadingProps {
    fn from(props: &GridSectionProps<B>) -> Self {
        let heading = props.section.heading;
        Self { heading }
    }
}

/// Resolves the section into the generic grid engine's props: the views the domain
/// rendered plus the callbacks that send the user's gestures back to the domain.
/// This makes no domain decision; it reads state, sends state, receives state, and
/// hands display values to the grid. The toast handle comes from context, used only
/// to warn when a move is refused.
impl<B: GridBehavior> From<&GridSectionProps<B>> for CommandGridProps {
    fn from(props: &GridSectionProps<B>) -> Self {
        let behavior = props.behavior.clone();
        let section = &props.section;
        let toast = consume_toast();
        let grid_id = section.heading;
        let race = section.race;
        let loaded_keys = section.loaded_keys;
        let grid_layout = section.grid_layout;
        let selected_slot = section.selected_slot;
        let selected_from_research = section.selected_from_research;
        let selected_from_uprooted = section.selected_from_uprooted;
        let tier_overrides = section.tier_overrides;
        let update_hotkeys_on_move = section.update_hotkeys_on_move;
        let hotkey_assign_request = section.hotkey_assign_request;
        let prevent_swap_on_drop = section.prevent_swap_on_drop;
        let slot_ids = section.slot_ids.clone();
        let restrict_draggable_to: Rc<[GridSlotId]> =
            Rc::from(section.restrict_draggable_to.as_slice());

        let views = {
            let read_guard = loaded_keys.read();
            match read_guard.as_ref() {
                Some(file) => {
                    let tier_guard = tier_overrides.read();
                    let input = CommandGridRenderInput {
                        slots: &slot_ids,
                        layout: *grid_layout.read(),
                        selected: *selected_slot.read(),
                        selected_is_research: *selected_from_research.read(),
                        tier_overrides: &tier_guard,
                        restrict_draggable_to: &restrict_draggable_to,
                    };
                    let rendered = file.rendered_command_grid(&behavior, &input);
                    let collected: Vec<GridTileView> = rendered.iter().map(tile_view).collect();
                    Rc::from(collected)
                }
                None => Rc::from(Vec::new()),
            }
        };

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
            grid_layout,
            selected_slot,
            update_hotkeys_on_move,
            prevent_swap_on_drop,
            slot_ids: slot_ids.clone(),
            toast,
        };
        let on_move = move_handler(move_args);
        let drop_blocked = drop_blocked_callback(behavior, loaded_keys, slot_ids);

        let dragging_slot = section.dragging_slot;
        let drop_target_tile = section.drop_target_tile;
        let drag_follower = section.drag_follower;
        Self {
            views,
            grid_id,
            race,
            dragging_slot,
            drop_target_tile,
            drag_follower,
            on_select,
            on_activate,
            on_move,
            drop_blocked,
        }
    }
}

/// Copies one domain-resolved tile onto the grid's display type. This is the only
/// adaptation the UI performs: a raw icon path becomes an asset URL and the
/// domain's flags pick the widget's visual enums. No decision is made here.
fn tile_view(rendered: &RenderedTile) -> GridTileView {
    let coordinate = rendered.coordinate();
    let icon = rendered
        .icon()
        .map(IconUrl::from_icon_path)
        .map(|url| url.to_string());
    let label = rendered.display_name().to_string();
    let hotkey = rendered.hotkey();
    let badge_state = if rendered.is_conflict() {
        HotkeyBadgeState::Conflict
    } else if rendered.is_passive() {
        HotkeyBadgeState::Passive
    } else {
        HotkeyBadgeState::Normal
    };
    let state = if rendered.occupant().is_none() {
        GridTileState::Empty
    } else if rendered.is_selected() {
        GridTileState::Selected
    } else if rendered.is_command() {
        GridTileState::Command
    } else {
        GridTileState::Filled
    };
    let flags = GridTileFlags::new(
        rendered.draggable(),
        rendered.is_command(),
        rendered.is_passive(),
    );
    GridTileView::new(coordinate, icon, label, hotkey, badge_state, state, flags)
}

fn occupant_at<B: GridBehavior>(
    behavior: &B,
    loaded_keys: Signal<Option<CustomKeys>>,
    slot_ids: &[GridSlotId],
    coordinate: GridCoordinate,
) -> Option<GridSlotId> {
    let read_guard = loaded_keys.read();
    let file = read_guard.as_ref()?;
    let column = u8::from(coordinate.column());
    let row = u8::from(coordinate.row());
    file.slot_at_position(slot_ids, behavior.research_positions(), column, row)
}

fn select_handler<B: GridBehavior>(
    behavior: B,
    loaded_keys: Signal<Option<CustomKeys>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
    mut selected_from_research: Signal<bool>,
    mut selected_from_uprooted: Signal<bool>,
    slot_ids: Rc<[GridSlotId]>,
) -> EventHandler<GridCoordinate> {
    EventHandler::new(move |coordinate: GridCoordinate| {
        let occupant = occupant_at(&behavior, loaded_keys, &slot_ids, coordinate);
        selected_slot.set(occupant);
        selected_from_research.set(behavior.research_positions());
        selected_from_uprooted.set(behavior.marks_alternate_form());
        FocusModality::after_render(".tile-override-card .override-key-cell");
    })
}

fn activate_handler<B: GridBehavior>(
    behavior: B,
    loaded_keys: Signal<Option<CustomKeys>>,
    mut selected_slot: Signal<Option<GridSlotId>>,
    mut selected_from_research: Signal<bool>,
    mut selected_from_uprooted: Signal<bool>,
    mut hotkey_assign_request: Signal<bool>,
    slot_ids: Rc<[GridSlotId]>,
) -> EventHandler<GridCoordinate> {
    EventHandler::new(move |coordinate: GridCoordinate| {
        let occupant = occupant_at(&behavior, loaded_keys, &slot_ids, coordinate);
        let Some(slot) = occupant else {
            return;
        };
        selected_slot.set(Some(slot));
        selected_from_research.set(behavior.research_positions());
        selected_from_uprooted.set(behavior.marks_alternate_form());
        hotkey_assign_request.set(true);
    })
}

struct MoveHandlerArgs<B: GridBehavior> {
    behavior: B,
    loaded_keys: Signal<Option<CustomKeys>>,
    grid_layout: Signal<GridLayout>,
    selected_slot: Signal<Option<GridSlotId>>,
    update_hotkeys_on_move: Signal<bool>,
    prevent_swap_on_drop: bool,
    slot_ids: Rc<[GridSlotId]>,
    toast: Toasts,
}

fn move_handler<B: GridBehavior>(args: MoveHandlerArgs<B>) -> EventHandler<Range<GridCoordinate>> {
    let MoveHandlerArgs {
        behavior,
        mut loaded_keys,
        grid_layout,
        mut selected_slot,
        update_hotkeys_on_move,
        prevent_swap_on_drop,
        slot_ids,
        toast,
    } = args;
    EventHandler::new(move |grid_move: Range<GridCoordinate>| {
        let from = grid_move.start;
        let to = grid_move.end;
        let moving_slot = occupant_at(&behavior, loaded_keys, &slot_ids, from);
        let Some(moving_slot) = moving_slot else {
            return;
        };
        let blocker_name = {
            let read_guard = loaded_keys.read();
            read_guard
                .as_ref()
                .and_then(|file| file.command_grid_move_blocker(&behavior, &slot_ids, from, to))
        };
        if let Some(blocker_name) = blocker_name {
            let message = format!("Slot reserved for {blocker_name}'s off-state");
            let options =
                ToastOptions::new().description("Reassign it via the override panel first.");
            toast.warning(message, options);
            selected_slot.set(Some(moving_slot));
            return;
        }
        let assign_hotkey_on_move = *update_hotkeys_on_move.read();
        let layout_snapshot = *grid_layout.read();
        let target_column = u8::from(to.column());
        let target_row = u8::from(to.row());
        let move_request = MoveRequest::new(
            layout_snapshot,
            &slot_ids,
            &moving_slot,
            target_column,
            target_row,
            behavior.research_positions(),
        )
        .with_prevent_swap(prevent_swap_on_drop)
        .with_prevent_co_move(!behavior.co_move_offstate())
        .with_assign_hotkey_on_move(assign_hotkey_on_move);
        Positions::move_or_swap(&mut loaded_keys, move_request);
        selected_slot.set(Some(moving_slot));
    })
}

fn drop_blocked_callback<B: GridBehavior>(
    behavior: B,
    loaded_keys: Signal<Option<CustomKeys>>,
    slot_ids: Rc<[GridSlotId]>,
) -> Callback<Range<GridCoordinate>, bool> {
    Callback::new(move |grid_move: Range<GridCoordinate>| {
        let read_guard = loaded_keys.read();
        let Some(file) = read_guard.as_ref() else {
            return false;
        };
        let blocker =
            file.command_grid_move_blocker(&behavior, &slot_ids, grid_move.start, grid_move.end);
        blocker.is_some()
    })
}
