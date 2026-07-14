use crate::components::app::components::shell::components::toasts::{ToastOptions, Toasts};
use dioxus::prelude::*;
use std::ops::Range;
use std::rc::Rc;

use warcraft_keybinds::{CustomKeys, GridBehavior, GridCoordinate, GridLayout, GridSlotId};

use crate::services::customkeys::service::CustomKeysService;
use warcraft_keybinds::MoveRequest;

pub(super) fn occupant_at<B: GridBehavior>(
    behavior: &B,
    loaded_keys: Signal<Option<CustomKeys>>,
    slot_ids: &[GridSlotId],
    coordinate: GridCoordinate,
) -> Option<GridSlotId> {
    let read_guard = loaded_keys.peek();
    let file = read_guard.as_ref()?;
    file.slot_at(behavior, slot_ids, coordinate)
}

pub(super) fn select_handler<B: GridBehavior>(
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
    })
}

pub(super) fn activate_handler<B: GridBehavior>(
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

pub(super) struct MoveHandlerArgs<B: GridBehavior> {
    pub(super) behavior: B,
    pub(super) loaded_keys: Signal<Option<CustomKeys>>,
    pub(super) custom_keys_service: CustomKeysService,
    pub(super) grid_layout: Signal<GridLayout>,
    pub(super) selected_slot: Signal<Option<GridSlotId>>,
    pub(super) update_hotkeys_on_move: Signal<bool>,
    pub(super) prevent_swap_on_drop: bool,
    pub(super) slot_ids: Rc<[GridSlotId]>,
    pub(super) toast: Toasts,
}

pub(super) fn move_handler<B: GridBehavior>(
    args: MoveHandlerArgs<B>,
) -> EventHandler<Range<GridCoordinate>> {
    let MoveHandlerArgs {
        behavior,
        loaded_keys,
        custom_keys_service,
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
            let read_guard = loaded_keys.peek();
            read_guard
                .as_ref()
                .and_then(|file| file.command_grid_move_blocker(&behavior, &slot_ids, from, to))
        };
        if let Some(blocker_name) = blocker_name {
            let message = format!("Slot reserved for {blocker_name}'s off-state");
            let options =
                ToastOptions::new().description("Reassign it via the hotkey override first.");
            toast.warning(message, options);
            selected_slot.set(Some(moving_slot));
            return;
        }
        let assign_hotkey_on_move = *update_hotkeys_on_move.read();
        let layout_snapshot = *grid_layout.read();
        let move_request =
            MoveRequest::for_behavior(&behavior, layout_snapshot, &slot_ids, &moving_slot, to)
                .with_prevent_swap(prevent_swap_on_drop)
                .with_assign_hotkey_on_move(assign_hotkey_on_move);
        custom_keys_service.move_slot(&move_request);
        selected_slot.set(Some(moving_slot));
    })
}

pub(super) fn drop_blocked_callback<B: GridBehavior>(
    behavior: B,
    loaded_keys: Signal<Option<CustomKeys>>,
    slot_ids: Rc<[GridSlotId]>,
) -> Callback<Range<GridCoordinate>, bool> {
    Callback::new(move |grid_move: Range<GridCoordinate>| {
        let read_guard = loaded_keys.peek();
        let Some(file) = read_guard.as_ref() else {
            return false;
        };
        let blocker =
            file.command_grid_move_blocker(&behavior, &slot_ids, grid_move.start, grid_move.end);
        blocker.is_some()
    })
}
