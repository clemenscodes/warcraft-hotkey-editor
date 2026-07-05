use super::drag_state::{
    DID_DRAG_MOVE, DRAG_MOVEMENT_THRESHOLD_PIXELS, DRAG_ORIGIN, DRAG_RAF_CLOSURE, DRAG_RAF_HANDLE,
    DragMovePoint, DragOrigin, DragRafClosure, DragThreadState, LATEST_DRAG_MOVE, LONG_PRESS_MS,
    PENDING_DRAG, PendingDragData, SUPPRESS_NEXT_CLICK, SUPPRESS_NEXT_DOUBLE_CLICK,
    TOUCH_CANCEL_THRESHOLD_PIXELS, TOUCH_LONG_PRESS_CLOSURE, TOUCH_LONG_PRESS_TIMER_ID,
    TOUCH_STARTED,
};

use crate::services::editor_state::{CursorPoint, HitTestPoint};
use crate::services::editor_state::{
    DragFollower, DragFollowerVisual, DraggingSlot, DropTargetTile,
};
use dioxus::html::input_data::MouseButton;
use dioxus::html::point_interaction::PointerInteraction;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::ops::Range;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, RowIndex};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(crate) fn keydown(
    on_select: EventHandler<GridCoordinate>,
    coordinate: GridCoordinate,
) -> impl FnMut(Event<KeyboardData>) + 'static {
    move |event: Event<KeyboardData>| {
        let key_value = event.data().key().to_string();
        if key_value == " " || key_value == "Enter" {
            event.prevent_default();
            on_select.call(coordinate);
        }
    }
}

pub(crate) struct PointerDownArgs {
    pub(crate) draggable: bool,
    pub(crate) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(crate) drop_target_tile: Signal<Option<DropTargetTile>>,
    pub(crate) drag_follower: Signal<Option<DragFollower>>,
    pub(crate) visual: Option<DragFollowerVisual>,
    pub(crate) grid_id: &'static str,
    pub(crate) coordinate: GridCoordinate,
}

pub(crate) fn pointer_down(args: PointerDownArgs) -> impl FnMut(Event<PointerData>) + 'static {
    let PointerDownArgs {
        draggable,
        mut dragging_slot,
        mut drop_target_tile,
        mut drag_follower,
        visual,
        grid_id,
        coordinate,
    } = args;
    move |event: Event<PointerData>| {
        SUPPRESS_NEXT_DOUBLE_CLICK.with(|cell| cell.set(false));
        if !draggable {
            return;
        }
        let Some(visual) = visual.clone() else {
            return;
        };
        if event.data().trigger_button() != Some(MouseButton::Primary) {
            return;
        }
        let Some(web_event) = event.data().try_as_web_event() else {
            return;
        };
        let pointer_type = web_event.pointer_type();
        let is_touch = pointer_type == "touch" || pointer_type == "pen";
        if !is_touch && TOUCH_STARTED.with(|c| c.replace(false)) {
            return;
        }
        DragThreadState::reset();
        if is_touch {
            TOUCH_STARTED.with(|c| c.set(true));
        }
        if dragging_slot.read().is_some() {
            dragging_slot.set(None);
            drop_target_tile.set(None);
            drag_follower.set(None);
        }
        let Some(target_node) = web_event.target() else {
            return;
        };
        let target_element_result: Result<web_sys::Element, _> = target_node.dyn_into();
        let Ok(target_element) = target_element_result else {
            return;
        };
        let tile_lookup = target_element.closest("[data-grid-row]");
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
        let pending = PendingDragData {
            grid_id,
            coordinate,
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
            let mut dragging_slot_cb = dragging_slot;
            let mut drop_target_tile_cb = drop_target_tile;
            let mut drag_follower_cb = drag_follower;
            let cb = Closure::once(move || {
                let Some(pending) = PENDING_DRAG.with(|cell| cell.borrow_mut().take()) else {
                    return;
                };
                if pending
                    .tile_element
                    .set_pointer_capture(pending.pointer_id)
                    .is_err()
                {
                    return;
                }
                DragThreadState::install_scroll_lock();
                DID_DRAG_MOVE.with(|c| c.set(true));
                let dragging = DraggingSlot::new(pending.grid_id, pending.coordinate);
                dragging_slot_cb.set(Some(dragging));
                let initial_target = DropTargetTile::new(pending.grid_id, pending.coordinate);
                drop_target_tile_cb.set(Some(initial_target));
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
                TOUCH_LONG_PRESS_CLOSURE.with(|c| *c.borrow_mut() = Some(cb));
            }
        }
    }
}

pub(crate) fn pointer_move(
    mut dragging_slot: Signal<Option<DraggingSlot>>,
    mut drop_target_tile: Signal<Option<DropTargetTile>>,
    mut drag_follower: Signal<Option<DragFollower>>,
    grid_id: &'static str,
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
                PENDING_DRAG.with(|cell| {
                    if let Some(pending) = cell.borrow_mut().as_mut() {
                        pending.last_cursor_horizontal_position = cursor_horizontal_position;
                        pending.last_cursor_vertical_position = cursor_vertical_position;
                    }
                });
                if !drag_is_active {
                    return;
                }
            } else {
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
                            let pending_grid_id = pending.grid_id;
                            let pending_coordinate = pending.coordinate;
                            let pending_visual = pending.visual;
                            let pending_click_offset_horizontal = pending.click_offset_horizontal;
                            let pending_click_offset_vertical = pending.click_offset_vertical;
                            let pending_tile_width = pending.tile_width;
                            let pending_tile_height = pending.tile_height;
                            let dragging = DraggingSlot::new(pending_grid_id, pending_coordinate);
                            dragging_slot.set(Some(dragging));
                            let initial_target =
                                DropTargetTile::new(pending_grid_id, pending_coordinate);
                            drop_target_tile.set(Some(initial_target));
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
        let point = DragMovePoint {
            client_horizontal: cursor_horizontal_position,
            client_vertical: cursor_vertical_position,
        };
        LATEST_DRAG_MOVE.with(|cell| cell.set(Some(point)));
        let frame_already_pending = DRAG_RAF_HANDLE.with(|cell| cell.get().is_some());
        if frame_already_pending {
            return;
        }
        let Some(window) = web_sys::window() else {
            return;
        };
        let closure: DragRafClosure = Closure::new(move |_timestamp: f64| {
            let Some(point) = LATEST_DRAG_MOVE.with(|cell| cell.take()) else {
                return;
            };
            DRAG_RAF_HANDLE.with(|cell| cell.set(None));
            flush_drag_move(
                dragging_slot,
                drop_target_tile,
                drag_follower,
                grid_id,
                point,
            );
        });
        if let Ok(handle) = window.request_animation_frame(closure.as_ref().unchecked_ref()) {
            DRAG_RAF_HANDLE.with(|cell| cell.set(Some(handle)));
        }
        DRAG_RAF_CLOSURE.with(|cell| *cell.borrow_mut() = Some(closure));
    }
}

fn flush_drag_move(
    _dragging_slot: Signal<Option<DraggingSlot>>,
    mut drop_target_tile: Signal<Option<DropTargetTile>>,
    mut drag_follower: Signal<Option<DragFollower>>,
    grid_id: &'static str,
    point: DragMovePoint,
) {
    let cursor_horizontal_position = point.client_horizontal;
    let cursor_vertical_position = point.client_vertical;
    let current_follower_option = drag_follower.read().clone();
    if let Some(mut current_follower) = current_follower_option {
        current_follower.set_cursor_position(cursor_horizontal_position, cursor_vertical_position);
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
        elem_under_option.and_then(|elem| elem.closest("[data-grid-row]").ok().flatten());
    let Some(tile_under) = tile_under_option else {
        if drop_target_tile.read().is_some() {
            drop_target_tile.set(None);
        }
        return;
    };
    let grid_id_host_option = tile_under.closest("[data-grid-id]").ok().flatten();
    let grid_id_attribute = grid_id_host_option.and_then(|host| host.get_attribute("data-grid-id"));
    let Some(grid_id_under) = grid_id_attribute else {
        if drop_target_tile.read().is_some() {
            drop_target_tile.set(None);
        }
        return;
    };
    if grid_id_under != grid_id {
        if drop_target_tile.read().is_some() {
            drop_target_tile.set(None);
        }
        return;
    }
    let row_attr = tile_under.get_attribute("data-grid-row");
    let col_attr = tile_under.get_attribute("data-grid-col");
    let Some(under_row) = row_attr
        .as_deref()
        .and_then(|raw| raw.parse::<u8>().ok())
        .and_then(|value| RowIndex::try_from(value).ok())
    else {
        return;
    };
    let Some(under_column) = col_attr
        .as_deref()
        .and_then(|raw| raw.parse::<u8>().ok())
        .and_then(|value| ColumnIndex::try_from(value).ok())
    else {
        return;
    };
    let under_coordinate = GridCoordinate::new(under_column, under_row);
    let new_target = DropTargetTile::new(grid_id, under_coordinate);
    let needs_update = drop_target_tile
        .read()
        .as_ref()
        .map(|existing| *existing != new_target)
        .unwrap_or(true);
    if needs_update {
        drop_target_tile.set(Some(new_target));
    }
}

pub(crate) struct PointerUpArgs {
    pub(crate) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(crate) drop_target_tile: Signal<Option<DropTargetTile>>,
    pub(crate) drag_follower: Signal<Option<DragFollower>>,
    pub(crate) on_move: EventHandler<Range<GridCoordinate>>,
    pub(crate) on_select: EventHandler<GridCoordinate>,
    pub(crate) grid_id: &'static str,
}

pub(crate) fn pointer_up(args: PointerUpArgs) -> impl FnMut(Event<PointerData>) + 'static {
    let PointerUpArgs {
        mut dragging_slot,
        mut drop_target_tile,
        mut drag_follower,
        on_move,
        on_select,
        grid_id,
    } = args;
    move |_event: Event<PointerData>| {
        DragThreadState::cancel_long_press();
        let dragging_clone = *dragging_slot.read();
        let mut committed = false;
        let mut fell_back = false;
        if let Some(dragging) = dragging_clone.as_ref()
            && dragging.grid_id() == grid_id
        {
            let drop_clone = *drop_target_tile.read();
            let valid_drop = drop_clone
                .filter(|drop| drop.grid_id() == grid_id)
                .filter(|drop| drop.coordinate() != dragging.coordinate());
            if let Some(drop) = valid_drop {
                let source_coordinate = dragging.coordinate();
                let target_coordinate = drop.coordinate();
                let grid_move = source_coordinate..target_coordinate;
                on_move.call(grid_move);
                committed = true;
            } else {
                fell_back = true;
            }
        }
        let did_move = DID_DRAG_MOVE.with(|cell| cell.replace(false));
        DRAG_ORIGIN.with(|cell| cell.set(None));
        PENDING_DRAG.with(|cell| *cell.borrow_mut() = None);
        DragThreadState::remove_scroll_lock();
        if fell_back
            && did_move
            && let Some(dragging) = dragging_clone.as_ref()
        {
            let source_coordinate = dragging.coordinate();
            on_select.call(source_coordinate);
        }
        if did_move || committed {
            SUPPRESS_NEXT_CLICK.with(|cell| cell.set(true));
            SUPPRESS_NEXT_DOUBLE_CLICK.with(|cell| cell.set(true));
        }
        dragging_slot.set(None);
        drop_target_tile.set(None);
        drag_follower.set(None);
    }
}

pub(crate) fn pointer_cancel(
    mut dragging_slot: Signal<Option<DraggingSlot>>,
    mut drop_target_tile: Signal<Option<DropTargetTile>>,
    mut drag_follower: Signal<Option<DragFollower>>,
) -> impl FnMut(Event<PointerData>) + 'static {
    move |_event: Event<PointerData>| {
        DragThreadState::reset();
        dragging_slot.set(None);
        drop_target_tile.set(None);
        drag_follower.set(None);
    }
}

pub(crate) fn lost_pointer_capture(
    mut dragging_slot: Signal<Option<DraggingSlot>>,
    mut drop_target_tile: Signal<Option<DropTargetTile>>,
    mut drag_follower: Signal<Option<DragFollower>>,
) -> impl FnMut(Event<PointerData>) + 'static {
    move |_event: Event<PointerData>| {
        DragThreadState::reset();
        dragging_slot.set(None);
        drop_target_tile.set(None);
        drag_follower.set(None);
    }
}

pub(crate) fn click(
    on_select: EventHandler<GridCoordinate>,
    coordinate: GridCoordinate,
) -> impl FnMut(Event<MouseData>) + 'static {
    move |_event: Event<MouseData>| {
        let was_suppressed = SUPPRESS_NEXT_CLICK.with(|suppress| suppress.replace(false));
        if was_suppressed {
            return;
        }
        on_select.call(coordinate);
    }
}

pub(crate) fn double_click(
    on_activate: EventHandler<GridCoordinate>,
    coordinate: GridCoordinate,
) -> impl FnMut(Event<MouseData>) + 'static {
    move |_event: Event<MouseData>| {
        let was_suppressed = SUPPRESS_NEXT_DOUBLE_CLICK.with(|suppress| suppress.replace(false));
        if was_suppressed {
            return;
        }
        on_activate.call(coordinate);
    }
}
