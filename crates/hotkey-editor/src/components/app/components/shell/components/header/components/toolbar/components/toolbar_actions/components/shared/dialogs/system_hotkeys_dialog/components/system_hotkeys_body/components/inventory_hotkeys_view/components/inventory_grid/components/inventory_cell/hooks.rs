use super::super::super::{
    DID_DRAG_MOVE, DRAG_MOVEMENT_THRESHOLD_PIXELS, DRAG_ORIGIN, DRAG_RAF_CLOSURE, DRAG_RAF_HANDLE,
    DragMovePoint, DragOrigin, DragRafClosure, InventoryDragFollower, InventoryDragRaf,
    InventoryDragSource, LATEST_DRAG_MOVE, SUPPRESS_NEXT_CLICK,
};

use super::props::InventoryCellProps;
use super::state::InventoryCellState;
use crate::services::editor_state::{CursorPoint, HitTestPoint};
use dioxus::html::input_data::MouseButton;
use dioxus::html::point_interaction::PointerInteraction;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::Cell;
use std::collections::HashMap;
use warcraft_keybinds::{CustomKeys, EffectiveBinding, KeyCode};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Everything the cell's markup needs, already shaped: its glow state, drag flag,
/// caption/key, conflict tooltip, whether its picker is open, and the full set of
/// pointer/click/pick handlers that drive the drag-to-swap and edit-on-click
/// behaviour. All of that work lives here so the body is pure.
pub(super) struct InventoryCellModel {
    pub(super) state: InventoryCellState,
    pub(super) slot_id: String,
    pub(super) dragging_attr: &'static str,
    pub(super) slot_label: String,
    pub(super) key_label: String,
    pub(super) conflict_title: String,
    pub(super) is_conflict: bool,
    pub(super) is_editing: bool,
    pub(super) current_code: KeyCode,
    pub(super) picker_conflicts: HashMap<KeyCode, Vec<String>>,
    pub(super) on_pointerdown: EventHandler<Event<PointerData>>,
    pub(super) on_pointermove: EventHandler<Event<PointerData>>,
    pub(super) on_pointerup: EventHandler<Event<PointerData>>,
    pub(super) on_pointercancel: EventHandler<Event<PointerData>>,
    pub(super) on_click: EventHandler<MouseEvent>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) on_close: EventHandler<()>,
}

/// Composes an inventory cell's state and behaviour.
pub(super) fn use_inventory_cell(props: &InventoryCellProps) -> InventoryCellModel {
    let loaded_keys = props.loaded_keys;
    let mut editing_section = props.editing_section;
    let mut dragging_source = props.dragging_source;
    let mut drop_target = props.drop_target;
    let mut drag_follower = props.drag_follower;
    let binding_map = props.binding_map;
    let mut keys_signal = loaded_keys;
    let section_id = props.section_id.clone();
    let default_hotkey = props.default_hotkey;
    let default_modifier = props.default_modifier;
    let slot_index = props.slot_index;
    let read_guard = loaded_keys.read();
    let effective = EffectiveBinding::resolve_from_file(
        read_guard.as_ref(),
        &section_id,
        default_hotkey,
        default_modifier,
    );
    drop(read_guard);
    let map_guard = binding_map.read();
    let collisions = map_guard.collisions_for(&section_id, effective.key(), effective.modifier());
    let is_conflict = !collisions.is_empty();
    let conflict_title = if is_conflict {
        let names: Vec<String> = collisions
            .iter()
            .map(|resolved| resolved.section_comment().to_string())
            .collect();
        format!("Also used by {}", names.join(", "))
    } else {
        String::new()
    };
    let picker_conflicts = map_guard.picker_conflicts(&section_id, effective.modifier());
    drop(map_guard);
    let is_editing = editing_section
        .read()
        .as_deref()
        .map(|active| active == section_id.as_str())
        .unwrap_or(false);
    let is_being_dragged = dragging_source
        .read()
        .as_ref()
        .map(|source| source.section_id == section_id)
        .unwrap_or(false);
    let is_drop_target = drop_target
        .read()
        .as_deref()
        .map(|target| target == section_id.as_str())
        .unwrap_or(false);
    let state = if is_conflict {
        InventoryCellState::Conflict
    } else if is_editing || is_drop_target {
        InventoryCellState::Active
    } else {
        InventoryCellState::Idle
    };
    let dragging_attr = if is_being_dragged { "true" } else { "false" };
    let key_label = if is_editing {
        String::from("…")
    } else {
        effective.label()
    };
    let slot_label = format!("Slot {}", slot_index + 1);
    let current_code = effective.key();
    let section_id_for_click = section_id.clone();
    let section_id_for_pick = section_id.clone();
    let section_id_for_pointerdown = section_id.clone();
    let section_id_for_pointermove = section_id.clone();
    let section_id_for_pointerup = section_id.clone();
    let label_for_drag = key_label.clone();
    let on_pointerdown = EventHandler::new(move |event: Event<PointerData>| {
        if event.data().trigger_button() != Some(MouseButton::Primary) {
            return;
        }
        let Some(web_event) = event.data().try_as_web_event() else {
            return;
        };
        let pointer_type = web_event.pointer_type();
        if pointer_type == "touch" || pointer_type == "pen" {
            return;
        }
        let Some(target_node) = web_event.target() else {
            return;
        };
        let target_element_result: Result<web_sys::Element, _> = target_node.dyn_into();
        let Ok(target_element) = target_element_result else {
            return;
        };
        let cell_lookup = target_element.closest(".inventory-cell");
        let Ok(Some(cell_element)) = cell_lookup else {
            return;
        };
        let cell_rect = cell_element.get_bounding_client_rect();
        let cursor_horizontal_position = f64::from(web_event.client_x());
        let cursor_vertical_position = f64::from(web_event.client_y());
        let click_offset_horizontal = cursor_horizontal_position - cell_rect.left();
        let click_offset_vertical = cursor_vertical_position - cell_rect.top();
        let pointer_id = web_event.pointer_id();
        let _ = cell_element.set_pointer_capture(pointer_id);
        let drag_origin = DragOrigin {
            cursor_horizontal_position,
            cursor_vertical_position,
        };
        DRAG_ORIGIN.with(|cell| cell.set(Some(drag_origin)));
        DID_DRAG_MOVE.with(|cell: &Cell<bool>| cell.set(false));
        let drag_source = InventoryDragSource {
            section_id: section_id_for_pointerdown.clone(),
        };
        dragging_source.set(Some(drag_source));
        drop_target.set(None);
        let follower = InventoryDragFollower {
            section_id: section_id_for_pointerdown.clone(),
            label: label_for_drag.clone(),
            click_offset_horizontal,
            click_offset_vertical,
            cursor_horizontal_position,
            cursor_vertical_position,
            width: cell_rect.width(),
            height: cell_rect.height(),
        };
        drag_follower.set(Some(follower));
    });
    let on_pointermove = EventHandler::new(move |event: Event<PointerData>| {
        if dragging_source.read().is_none() {
            return;
        }
        let Some(web_event) = event.data().try_as_web_event() else {
            return;
        };
        let cursor_horizontal_position = f64::from(web_event.client_x());
        let cursor_vertical_position = f64::from(web_event.client_y());
        if let Some(origin) = DRAG_ORIGIN.with(|cell| cell.get()) {
            let horizontal_delta = cursor_horizontal_position - origin.cursor_horizontal_position;
            let vertical_delta = cursor_vertical_position - origin.cursor_vertical_position;
            let distance_squared =
                horizontal_delta * horizontal_delta + vertical_delta * vertical_delta;
            let threshold_squared = DRAG_MOVEMENT_THRESHOLD_PIXELS * DRAG_MOVEMENT_THRESHOLD_PIXELS;
            if distance_squared > threshold_squared {
                DID_DRAG_MOVE.with(|cell: &Cell<bool>| cell.set(true));
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
        let section_id_for_flush = section_id_for_pointermove.clone();
        let closure: DragRafClosure = Closure::new(move |_timestamp: f64| {
            DRAG_RAF_HANDLE.with(|cell| cell.set(None));
            let Some(point) = LATEST_DRAG_MOVE.with(|cell| cell.take()) else {
                return;
            };
            flush_inventory_drag_move(
                drop_target,
                drag_follower,
                section_id_for_flush.clone(),
                point,
            );
        });
        if let Ok(handle) = window.request_animation_frame(closure.as_ref().unchecked_ref()) {
            DRAG_RAF_HANDLE.with(|cell| cell.set(Some(handle)));
        }
        DRAG_RAF_CLOSURE.with(|cell| *cell.borrow_mut() = Some(closure));
    });
    let on_pointerup = EventHandler::new(move |_event: Event<PointerData>| {
        if let Some(final_point) = LATEST_DRAG_MOVE.with(|cell| cell.take()) {
            flush_inventory_drag_move(
                drop_target,
                drag_follower,
                section_id_for_pointerup.clone(),
                final_point,
            );
        }
        InventoryDragRaf::cancel();
        let drop_clone = drop_target.read().clone();
        let mut performed_swap = false;
        if let Some(target_id) = drop_clone
            && target_id != section_id_for_pointerup
        {
            keys_signal
                .write()
                .get_or_insert_with(CustomKeys::default)
                .swap_system_bindings(&section_id_for_pointerup, &target_id);
            performed_swap = true;
        }
        let did_move = DID_DRAG_MOVE.with(|cell: &Cell<bool>| cell.replace(false));
        DRAG_ORIGIN.with(|cell| cell.set(None));
        if did_move || performed_swap {
            SUPPRESS_NEXT_CLICK.with(|cell: &Cell<bool>| cell.set(true));
        }
        dragging_source.set(None);
        drop_target.set(None);
        drag_follower.set(None);
    });
    let on_pointercancel = EventHandler::new(move |_event: Event<PointerData>| {
        InventoryDragRaf::cancel();
        DID_DRAG_MOVE.with(|cell: &Cell<bool>| cell.set(false));
        DRAG_ORIGIN.with(|cell| cell.set(None));
        dragging_source.set(None);
        drop_target.set(None);
        drag_follower.set(None);
    });
    let on_click = EventHandler::new(move |_event: MouseEvent| {
        if SUPPRESS_NEXT_CLICK.with(|cell: &Cell<bool>| cell.replace(false)) {
            return;
        }
        editing_section.set(Some(section_id_for_click.clone()));
    });
    let on_pick = EventHandler::new(move |code: KeyCode| {
        let mut guard = keys_signal.write();
        let file = guard.get_or_insert_with(CustomKeys::default);
        file.set_system_hotkey(&section_id_for_pick, code);
        drop(guard);
        editing_section.set(None);
    });
    let on_close = EventHandler::new(move |_event: ()| editing_section.set(None));
    InventoryCellModel {
        state,
        slot_id: section_id,
        dragging_attr,
        slot_label,
        key_label,
        conflict_title,
        is_conflict,
        is_editing,
        current_code,
        picker_conflicts,
        on_pointerdown,
        on_pointermove,
        on_pointerup,
        on_pointercancel,
        on_click,
        on_pick,
        on_close,
    }
}

fn flush_inventory_drag_move(
    mut drop_target: Signal<Option<String>>,
    mut drag_follower: Signal<Option<InventoryDragFollower>>,
    section_id: String,
    point: DragMovePoint,
) {
    let cursor_horizontal_position = point.client_horizontal;
    let cursor_vertical_position = point.client_vertical;
    let current_follower_option = drag_follower.read().clone();
    if let Some(mut current_follower) = current_follower_option {
        current_follower.cursor_horizontal_position = cursor_horizontal_position;
        current_follower.cursor_vertical_position = cursor_vertical_position;
        drag_follower.set(Some(current_follower));
    }
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let cursor_point = CursorPoint::new(cursor_horizontal_position, cursor_vertical_position);
    let hit_test_point = HitTestPoint::from(cursor_point);
    let hit_test_horizontal = hit_test_point.horizontal_position();
    let hit_test_vertical = hit_test_point.vertical_position();
    let elem_under_option = document.element_from_point(hit_test_horizontal, hit_test_vertical);
    let cell_under_option =
        elem_under_option.and_then(|elem| elem.closest(".inventory-cell").ok().flatten());
    let Some(cell_under) = cell_under_option else {
        if drop_target.read().is_some() {
            drop_target.set(None);
        }
        return;
    };
    let target_id = cell_under.get_attribute("data-inventory-slot");
    let Some(target_id_string) = target_id else {
        if drop_target.read().is_some() {
            drop_target.set(None);
        }
        return;
    };
    if target_id_string == section_id {
        if drop_target.read().is_some() {
            drop_target.set(None);
        }
        return;
    }
    let needs_update = drop_target
        .read()
        .as_deref()
        .map(|existing| existing != target_id_string.as_str())
        .unwrap_or(true);
    if needs_update {
        drop_target.set(Some(target_id_string));
    }
}
