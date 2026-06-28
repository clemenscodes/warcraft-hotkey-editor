use std::rc::Rc;

use dioxus::html::input_data::MouseButton;
use dioxus::html::point_interaction::PointerInteraction;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use dioxus_primitives::toast::{ToastOptions, Toasts};
use warcraft_database::ObjectLookup;
use warcraft_keybinds::CustomKeys;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::model::grid::GridLayout;
use crate::model::grid::{CursorPoint, HitTestPoint};
use crate::model::grid::{
    DragFollower, DragFollowerVisual, DraggingSlot, DropTargetCell, GridSlotId,
};
use crate::model::icons::IconUrl;
use crate::services::customkeys::positions::{MoveRequest, Positions};
use crate::services::focus::modality::FocusModality;

use super::drag_state::{
    DID_DRAG_MOVE, DRAG_MOVEMENT_THRESHOLD_PIXELS, DRAG_ORIGIN, DragOrigin, DragThreadState,
    LONG_PRESS_MS, PENDING_DRAG, PendingDragData, SUPPRESS_NEXT_CLICK, SUPPRESS_NEXT_DOUBLE_CLICK,
    TOUCH_CANCEL_THRESHOLD_PIXELS, TOUCH_LONG_PRESS_TIMER_ID, TOUCH_STARTED,
};

pub(super) fn keydown(
    mut selected_slot: Signal<Option<GridSlotId>>,
    occupant_for_keydown: Option<GridSlotId>,
    mut selected_from_research: Signal<bool>,
    is_research_grid: bool,
    mut selected_from_uprooted: Signal<bool>,
    is_uprooted_grid: bool,
) -> impl FnMut(Event<KeyboardData>) + 'static {
    move |event: Event<KeyboardData>| {
        let key_value = event.data().key().to_string();
        if key_value == " " || key_value == "Enter" {
            event.prevent_default();
            selected_slot.set(occupant_for_keydown);
            selected_from_research.set(is_research_grid);
            selected_from_uprooted.set(is_uprooted_grid);
            FocusModality::after_render(".tile-override-card .override-key-cell");
        }
    }
}

pub(super) struct PointerDownArgs {
    pub(super) tile_is_draggable: bool,
    pub(super) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(super) drop_target_cell: Signal<Option<DropTargetCell>>,
    pub(super) drag_follower: Signal<Option<DragFollower>>,
    pub(super) occupant_slot: Option<GridSlotId>,
    pub(super) restrict_draggable_to: Rc<[GridSlotId]>,
    pub(super) icon_src_option: Option<IconUrl>,
    pub(super) label_text: String,
    pub(super) displayed_letter: Option<String>,
    pub(super) is_passive_on_command_grid: bool,
    pub(super) is_command_cell: bool,
    pub(super) heading_text: &'static str,
    pub(super) column: u8,
    pub(super) row: u8,
}

pub(super) fn pointer_down(args: PointerDownArgs) -> impl FnMut(Event<PointerData>) + 'static {
    let PointerDownArgs {
        tile_is_draggable,
        mut dragging_slot,
        mut drop_target_cell,
        mut drag_follower,
        occupant_slot,
        restrict_draggable_to,
        icon_src_option,
        label_text,
        displayed_letter,
        is_passive_on_command_grid,
        is_command_cell,
        heading_text,
        column,
        row,
    } = args;
    move |event: Event<PointerData>| {
        // A new gesture starts: clear any stale double-click suppression so an
        // unrelated later double-click still opens the picker.
        SUPPRESS_NEXT_DOUBLE_CLICK.with(|cell| cell.set(false));
        if !tile_is_draggable {
            return;
        }
        if event.data().trigger_button() != Some(MouseButton::Primary) {
            return;
        }
        let Some(web_event) = event.data().try_as_web_event() else {
            return;
        };
        let pointer_type = web_event.pointer_type();
        let is_touch = pointer_type == "touch" || pointer_type == "pen";

        // Discard compat mouse event synthesised after touch.
        if !is_touch && TOUCH_STARTED.with(|c| c.replace(false)) {
            return;
        }
        // Clean up any stuck drag state from a previous gesture.
        DragThreadState::reset();
        // Flag so the compat mouse event is suppressed, but
        // continue handling the touch event itself.
        if is_touch {
            TOUCH_STARTED.with(|c| c.set(true));
        }
        if dragging_slot.read().is_some() {
            dragging_slot.set(None);
            drop_target_cell.set(None);
            drag_follower.set(None);
        }
        let Some(source_slot) = occupant_slot else {
            return;
        };
        // Picker mode: drag origin must
        // match the allow-list. Used by
        // the off-state picker so the
        // dialog only lets the player
        // grab the toggle's off half,
        // not the unit's other slots.
        if !restrict_draggable_to.is_empty()
            && !restrict_draggable_to
                .iter()
                .any(|allowed| allowed == &source_slot)
        {
            return;
        }
        let Some(target_node) = web_event.target() else {
            return;
        };
        let target_element_result: Result<web_sys::Element, _> = target_node.dyn_into();
        let Ok(target_element) = target_element_result else {
            return;
        };
        let tile_lookup = target_element.closest(".grid-tile");
        let Ok(Some(tile_element)) = tile_lookup else {
            return;
        };
        let tile_rect = tile_element.get_bounding_client_rect();
        let cursor_horizontal_position = f64::from(web_event.client_x());
        let cursor_vertical_position = f64::from(web_event.client_y());
        let click_offset_horizontal = cursor_horizontal_position - tile_rect.left();
        let click_offset_vertical = cursor_vertical_position - tile_rect.top();
        let tile_width = tile_rect.width();
        let tile_height = tile_rect.height();
        let pointer_id = web_event.pointer_id();

        let drag_origin = DragOrigin {
            cursor_horizontal_position,
            cursor_vertical_position,
        };
        DRAG_ORIGIN.with(|cell| cell.set(Some(drag_origin)));
        DID_DRAG_MOVE.with(|cell| cell.set(false));

        let visual = DragFollowerVisual::new(
            icon_src_option.clone(),
            label_text.clone(),
            displayed_letter.clone(),
            is_passive_on_command_grid,
            is_command_cell,
        );
        let pending = PendingDragData {
            source_slot,
            section: heading_text,
            column,
            row,
            visual,
            click_offset_horizontal,
            click_offset_vertical,
            tile_width,
            tile_height,
            tile_element,
            pointer_id,
            last_cursor_horizontal_position: cursor_horizontal_position,
            last_cursor_vertical_position: cursor_vertical_position,
            is_touch,
        };
        PENDING_DRAG.with(|cell| *cell.borrow_mut() = Some(pending));

        if is_touch {
            // Long-press: start timer. The closure captures
            // signal handles (Copy) to commit drag state when
            // the 300 ms window expires.
            let mut dragging_slot_cb = dragging_slot;
            let mut drop_target_cell_cb = drop_target_cell;
            let mut drag_follower_cb = drag_follower;
            let cb = Closure::once(move || {
                let Some(pending) = PENDING_DRAG.with(|cell| cell.borrow_mut().take()) else {
                    return;
                };
                // If capture fails the finger is already gone.
                if pending
                    .tile_element
                    .set_pointer_capture(pending.pointer_id)
                    .is_err()
                {
                    return;
                }
                DragThreadState::install_scroll_lock();
                DID_DRAG_MOVE.with(|c| c.set(true));
                let dragging = DraggingSlot::new(pending.source_slot, pending.section);
                dragging_slot_cb.set(Some(dragging));
                let initial_target =
                    DropTargetCell::new(pending.section, pending.column, pending.row);
                drop_target_cell_cb.set(Some(initial_target));
                let follower = DragFollower::new(
                    pending.visual,
                    pending.click_offset_horizontal,
                    pending.click_offset_vertical,
                    pending.last_cursor_horizontal_position,
                    pending.last_cursor_vertical_position,
                    pending.tile_width,
                    pending.tile_height,
                );
                drag_follower_cb.set(Some(follower));
            });
            if let Some(window) = web_sys::window()
                && let Ok(timer_id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    LONG_PRESS_MS,
                )
            {
                TOUCH_LONG_PRESS_TIMER_ID.with(|c| c.set(Some(timer_id)));
            }
            cb.forget();
        }
        // For mouse: drag commits in pointermove once the
        // movement threshold is crossed.
    }
}

pub(super) fn pointer_move(
    mut dragging_slot: Signal<Option<DraggingSlot>>,
    mut drop_target_cell: Signal<Option<DropTargetCell>>,
    mut drag_follower: Signal<Option<DragFollower>>,
    heading_text: &'static str,
) -> impl FnMut(Event<PointerData>) + 'static {
    move |event: Event<PointerData>| {
        let has_pending = PENDING_DRAG.with(|cell| cell.borrow().is_some());
        let drag_is_active = dragging_slot.read().is_some();
        if !has_pending && !drag_is_active {
            return;
        }
        let Some(web_event) = event.data().try_as_web_event() else {
            return;
        };
        let cursor_horizontal_position = f64::from(web_event.client_x());
        let cursor_vertical_position = f64::from(web_event.client_y());

        if has_pending {
            // Reject stale pending from a previous gesture whose
            // pointerup fired outside a tile (pointer_id mismatch).
            let current_pointer_id = web_event.pointer_id();
            let pending_pointer_id =
                PENDING_DRAG.with(|cell| cell.borrow().as_ref().map(|pending| pending.pointer_id));
            if pending_pointer_id != Some(current_pointer_id) {
                DragThreadState::cancel_long_press();
                PENDING_DRAG.with(|cell| *cell.borrow_mut() = None);
                DRAG_ORIGIN.with(|cell| cell.set(None));
                return;
            }

            let pending_is_touch = PENDING_DRAG.with(|cell| {
                cell.borrow()
                    .as_ref()
                    .map(|pending| pending.is_touch)
                    .unwrap_or(false)
            });

            if pending_is_touch {
                // Touch pending: cancel long-press if the finger
                // drifted far enough to be a swipe.
                let origin_option = DRAG_ORIGIN.with(|cell| cell.get());
                if let Some(origin) = origin_option {
                    let horizontal_delta =
                        cursor_horizontal_position - origin.cursor_horizontal_position;
                    let vertical_delta = cursor_vertical_position - origin.cursor_vertical_position;
                    if horizontal_delta * horizontal_delta + vertical_delta * vertical_delta
                        > TOUCH_CANCEL_THRESHOLD_PIXELS * TOUCH_CANCEL_THRESHOLD_PIXELS
                    {
                        DragThreadState::cancel_long_press();
                        PENDING_DRAG.with(|cell| *cell.borrow_mut() = None);
                        DRAG_ORIGIN.with(|cell| cell.set(None));
                        return;
                    }
                }
                // Keep last known position fresh so the follower
                // appears at the right spot when the timer fires.
                PENDING_DRAG.with(|cell| {
                    if let Some(p) = cell.borrow_mut().as_mut() {
                        p.last_cursor_horizontal_position = cursor_horizontal_position;
                        p.last_cursor_vertical_position = cursor_vertical_position;
                    }
                });
                // Drag not yet committed; wait for timer.
                if !drag_is_active {
                    return;
                }
            } else {
                // Mouse pending: commit on movement threshold.
                let origin_option = DRAG_ORIGIN.with(|cell| cell.get());
                if let Some(origin) = origin_option {
                    let horizontal_delta =
                        cursor_horizontal_position - origin.cursor_horizontal_position;
                    let vertical_delta = cursor_vertical_position - origin.cursor_vertical_position;
                    let distance_squared =
                        horizontal_delta * horizontal_delta + vertical_delta * vertical_delta;
                    let threshold_squared =
                        DRAG_MOVEMENT_THRESHOLD_PIXELS * DRAG_MOVEMENT_THRESHOLD_PIXELS;
                    if distance_squared > threshold_squared {
                        DID_DRAG_MOVE.with(|cell| cell.set(true));
                        let pending_option = PENDING_DRAG.with(|cell| cell.borrow_mut().take());
                        if let Some(pending) = pending_option {
                            if pending
                                .tile_element
                                .set_pointer_capture(pending.pointer_id)
                                .is_err()
                            {
                                DID_DRAG_MOVE.with(|cell| cell.set(false));
                                DRAG_ORIGIN.with(|cell| cell.set(None));
                                return;
                            }
                            let pending_source_slot = pending.source_slot;
                            let pending_section = pending.section;
                            let pending_column = pending.column;
                            let pending_row = pending.row;
                            let pending_visual = pending.visual;
                            let pending_click_offset_horizontal = pending.click_offset_horizontal;
                            let pending_click_offset_vertical = pending.click_offset_vertical;
                            let pending_tile_width = pending.tile_width;
                            let pending_tile_height = pending.tile_height;
                            let dragging = DraggingSlot::new(pending_source_slot, pending_section);
                            dragging_slot.set(Some(dragging));
                            let initial_target =
                                DropTargetCell::new(pending_section, pending_column, pending_row);
                            drop_target_cell.set(Some(initial_target));
                            let follower = DragFollower::new(
                                pending_visual,
                                pending_click_offset_horizontal,
                                pending_click_offset_vertical,
                                cursor_horizontal_position,
                                cursor_vertical_position,
                                pending_tile_width,
                                pending_tile_height,
                            );
                            drag_follower.set(Some(follower));
                        }
                    }
                }
                if dragging_slot.read().is_none() {
                    return;
                }
            }
        }

        let current_follower_option = drag_follower.read().clone();
        if let Some(mut current_follower) = current_follower_option {
            current_follower
                .set_cursor_position(cursor_horizontal_position, cursor_vertical_position);
            drag_follower.set(Some(current_follower));
        }

        let document_option = web_sys::window().and_then(|window| window.document());
        let Some(document) = document_option else {
            return;
        };
        let cursor_point = CursorPoint::new(cursor_horizontal_position, cursor_vertical_position);
        let hit_test_point = HitTestPoint::from(cursor_point);
        let hit_test_horizontal = hit_test_point.horizontal_position();
        let hit_test_vertical = hit_test_point.vertical_position();
        let elem_under_option = document.element_from_point(hit_test_horizontal, hit_test_vertical);
        let tile_under_option =
            elem_under_option.and_then(|elem| elem.closest(".grid-tile").ok().flatten());
        let Some(tile_under) = tile_under_option else {
            if drop_target_cell.read().is_some() {
                drop_target_cell.set(None);
            }
            return;
        };
        let section_attr = tile_under.get_attribute("data-grid-section");
        let Some(section_string) = section_attr else {
            if drop_target_cell.read().is_some() {
                drop_target_cell.set(None);
            }
            return;
        };
        if section_string != heading_text {
            if drop_target_cell.read().is_some() {
                drop_target_cell.set(None);
            }
            return;
        }
        let row_attr = tile_under.get_attribute("data-grid-row");
        let col_attr = tile_under.get_attribute("data-grid-col");
        let Some(under_row) = row_attr.as_deref().and_then(|raw| raw.parse::<u8>().ok()) else {
            return;
        };
        let Some(under_column) = col_attr.as_deref().and_then(|raw| raw.parse::<u8>().ok()) else {
            return;
        };
        let new_target = DropTargetCell::new(heading_text, under_column, under_row);
        let needs_update = drop_target_cell
            .read()
            .as_ref()
            .map(|existing| *existing != new_target)
            .unwrap_or(true);
        if needs_update {
            drop_target_cell.set(Some(new_target));
        }
    }
}

pub(super) struct PointerUpArgs {
    pub(super) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(super) drop_target_cell: Signal<Option<DropTargetCell>>,
    pub(super) drag_follower: Signal<Option<DragFollower>>,
    pub(super) heading_text: &'static str,
    pub(super) column: u8,
    pub(super) row: u8,
    pub(super) keys_signal: Signal<Option<CustomKeys>>,
    pub(super) slot_ids_for_drop: Rc<[GridSlotId]>,
    pub(super) is_research_grid: bool,
    pub(super) toast_api: Toasts,
    pub(super) update_hotkeys_on_move: Signal<bool>,
    pub(super) layout_snapshot: GridLayout,
    pub(super) prevent_swap_on_drop: bool,
    pub(super) is_uprooted_grid: bool,
    pub(super) selected_slot: Signal<Option<GridSlotId>>,
    pub(super) selected_from_research: Signal<bool>,
    pub(super) selected_from_uprooted: Signal<bool>,
}

pub(super) fn pointer_up(args: PointerUpArgs) -> impl FnMut(Event<PointerData>) + 'static {
    let PointerUpArgs {
        mut dragging_slot,
        mut drop_target_cell,
        mut drag_follower,
        heading_text,
        column,
        row,
        mut keys_signal,
        slot_ids_for_drop,
        is_research_grid,
        toast_api,
        update_hotkeys_on_move,
        layout_snapshot,
        prevent_swap_on_drop,
        is_uprooted_grid,
        mut selected_slot,
        mut selected_from_research,
        mut selected_from_uprooted,
    } = args;
    move |_event: Event<PointerData>| {
        // Cancel pending long-press if the finger lifted before
        // the timer fired (tap → select).
        DragThreadState::cancel_long_press();
        let dragging_clone = dragging_slot.read().clone();
        let mut performed_swap = false;
        let mut fell_back_to_source = false;
        if let Some(dragging) = dragging_clone.as_ref()
            && dragging.source_section() == heading_text
        {
            let drop_clone = *drop_target_cell.read();
            let valid_drop = drop_clone
                .filter(|drop| drop.section() == heading_text)
                .filter(|drop| drop.column() != column || drop.row() != row);
            if let Some(drop) = valid_drop {
                // Check whether the target is reserved by
                // another ability's off-state before committing.
                let moving_id = dragging.slot_id().as_str();
                let blocking_name = {
                    let keys = keys_signal.read();
                    let custom_keys = keys.as_ref();
                    // If the target already has an on-state
                    // occupant, this is a normal swap —
                    // move_or_swap will co-move the off-state.
                    let target_occupied = Positions::cell_for_position(
                        &slot_ids_for_drop,
                        custom_keys,
                        is_research_grid,
                        drop.column(),
                        drop.row(),
                    )
                    .is_some();
                    if target_occupied {
                        None
                    } else {
                        slot_ids_for_drop.iter().find_map(|slot| {
                            let GridSlotId::Ability(id) = slot else {
                                return None;
                            };
                            if id.value().eq_ignore_ascii_case(moving_id) {
                                return None;
                            }
                            let bound_id = *id;
                            let off_pos =
                                Positions::current_for_ability_off(bound_id, custom_keys)?;
                            let off_column = u8::from(off_pos.column());
                            let off_row = u8::from(off_pos.row());
                            if off_column == drop.column() && off_row == drop.row() {
                                Some(
                                    ObjectLookup::by_id(id.value())
                                        .and_then(|obj| obj.names().first().copied())
                                        .unwrap_or(id.value())
                                        .to_owned(),
                                )
                            } else {
                                None
                            }
                        })
                    }
                };
                if let Some(name) = blocking_name {
                    toast_api.warning(
                        format!("Slot reserved for {name}'s off-state"),
                        ToastOptions::new()
                            .description("Reassign it via the override panel first."),
                    );
                    fell_back_to_source = true;
                } else {
                    let assign_hotkey_on_move = *update_hotkeys_on_move.read();
                    let move_request = MoveRequest::new(
                        layout_snapshot,
                        &slot_ids_for_drop,
                        dragging.slot_id(),
                        drop.column(),
                        drop.row(),
                        is_research_grid,
                    )
                    .with_prevent_swap(prevent_swap_on_drop)
                    .with_prevent_co_move(is_uprooted_grid)
                    .with_assign_hotkey_on_move(assign_hotkey_on_move);
                    Positions::move_or_swap(&mut keys_signal, move_request);
                    let moved_slot = *dragging.slot_id();
                    selected_slot.set(Some(moved_slot));
                    performed_swap = true;
                }
            } else {
                fell_back_to_source = true;
            }
        }
        let did_move = DID_DRAG_MOVE.with(|cell| cell.replace(false));
        DRAG_ORIGIN.with(|cell| cell.set(None));
        PENDING_DRAG.with(|cell| *cell.borrow_mut() = None);
        DragThreadState::remove_scroll_lock();
        if fell_back_to_source
            && did_move
            && let Some(dragging) = dragging_clone.as_ref()
        {
            let source_slot = *dragging.slot_id();
            selected_slot.set(Some(source_slot));
            selected_from_research.set(is_research_grid);
            selected_from_uprooted.set(is_uprooted_grid);
        }
        if did_move || performed_swap {
            SUPPRESS_NEXT_CLICK.with(|cell| cell.set(true));
            // A drag happened, so the trailing native `dblclick` (prior click +
            // this gesture's click) must not open the picker.
            SUPPRESS_NEXT_DOUBLE_CLICK.with(|cell| cell.set(true));
        }
        dragging_slot.set(None);
        drop_target_cell.set(None);
        drag_follower.set(None);
    }
}

pub(super) fn pointer_cancel(
    mut dragging_slot: Signal<Option<DraggingSlot>>,
    mut drop_target_cell: Signal<Option<DropTargetCell>>,
    mut drag_follower: Signal<Option<DragFollower>>,
) -> impl FnMut(Event<PointerData>) + 'static {
    move |_event: Event<PointerData>| {
        DragThreadState::reset();
        dragging_slot.set(None);
        drop_target_cell.set(None);
        drag_follower.set(None);
    }
}

pub(super) fn lost_pointer_capture(
    mut dragging_slot: Signal<Option<DraggingSlot>>,
    mut drop_target_cell: Signal<Option<DropTargetCell>>,
    mut drag_follower: Signal<Option<DragFollower>>,
) -> impl FnMut(Event<PointerData>) + 'static {
    move |_event: Event<PointerData>| {
        // Safety net: fires whenever pointer capture is released
        // (after pointerup, pointercancel, or browser scroll
        // takeover). Ensures drag state never stays stuck.
        DragThreadState::reset();
        dragging_slot.set(None);
        drop_target_cell.set(None);
        drag_follower.set(None);
    }
}

pub(super) fn click(
    mut selected_slot: Signal<Option<GridSlotId>>,
    occupant_for_click: Option<GridSlotId>,
    mut selected_from_research: Signal<bool>,
    is_research_grid: bool,
    mut selected_from_uprooted: Signal<bool>,
    is_uprooted_grid: bool,
) -> impl FnMut(Event<MouseData>) + 'static {
    move |_event: Event<MouseData>| {
        let was_suppressed = SUPPRESS_NEXT_CLICK.with(|cell| cell.replace(false));
        if was_suppressed {
            return;
        }
        selected_slot.set(occupant_for_click);
        selected_from_research.set(is_research_grid);
        selected_from_uprooted.set(is_uprooted_grid);
    }
}

pub(super) fn double_click(
    mut selected_slot: Signal<Option<GridSlotId>>,
    occupant_for_dblclick: Option<GridSlotId>,
    mut selected_from_research: Signal<bool>,
    is_research_grid: bool,
    mut selected_from_uprooted: Signal<bool>,
    is_uprooted_grid: bool,
    mut hotkey_assign_request: Signal<bool>,
) -> impl FnMut(Event<MouseData>) + 'static {
    move |_event: Event<MouseData>| {
        // A drag that just ended produces a native dblclick; that must not open
        // the picker (initiating a drag resets the double-click trigger).
        let was_suppressed = SUPPRESS_NEXT_DOUBLE_CLICK.with(|cell| cell.replace(false));
        if was_suppressed {
            return;
        }
        let Some(slot) = occupant_for_dblclick else {
            return;
        };
        selected_slot.set(Some(slot));
        selected_from_research.set(is_research_grid);
        selected_from_uprooted.set(is_uprooted_grid);
        hotkey_assign_request.set(true);
    }
}
