use super::super::super::{
    DID_DRAG_MOVE, DRAG_MOVEMENT_THRESHOLD_PIXELS, DRAG_ORIGIN, DRAG_RAF_CLOSURE, DRAG_RAF_HANDLE,
    DragMovePoint, DragOrigin, DragRafClosure, InventoryDragFollower, InventoryDragRaf,
    InventoryDragSource, LATEST_DRAG_MOVE, SUPPRESS_NEXT_CLICK,
};

use super::logic::InventoryCellView;
use super::props::InventoryCellProps;
use super::state::InventoryCellState;
use crate::services::customkeys::context::use_custom_keys_service;
use dioxus::html::input_data::MouseButton;
use dioxus::html::point_interaction::PointerInteraction;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::Cell;
use std::collections::HashMap;
use warcraft_keybinds::KeyCode;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Everything the cell's markup needs, already shaped: its glow state, drag flag,
/// caption/key, conflict tooltip, whether its picker is open, and the full set of
/// pointer/click/pick handlers that drive the drag-to-swap and edit-on-click
/// behaviour. All of that work lives here so the body is pure.
pub(super) struct InventoryCellModel {
    pub(super) state: InventoryCellState,
    pub(super) slot_id: &'static str,
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

/// The pointer-drag handlers that drive the drag-to-swap gesture. Owns no signals of
/// its own — it drives the shared drag signals plus the thread-local drag session —
/// and commits a completed swap through the [`CustomKeysService`](crate::services::customkeys).
struct InventoryDrag {
    on_pointerdown: EventHandler<Event<PointerData>>,
    on_pointermove: EventHandler<Event<PointerData>>,
    on_pointerup: EventHandler<Event<PointerData>>,
    on_pointercancel: EventHandler<Event<PointerData>>,
}

/// The click-to-edit handlers: open the picker on click, commit a picked key through
/// the service, and close on dismiss.
struct InventoryEditing {
    on_click: EventHandler<MouseEvent>,
    on_pick: EventHandler<KeyCode>,
    on_close: EventHandler<()>,
}

fn use_inventory_drag(props: &InventoryCellProps, label_for_drag: String) -> InventoryDrag {
    let custom_keys_service = use_custom_keys_service();
    let mut dragging_source = props.dragging_source;
    let mut drop_target = props.drop_target;
    let mut drag_follower = props.drag_follower;
    let section_id_for_pointerdown = props.section_id;
    let section_id_for_pointermove = props.section_id;
    let section_id_for_pointerup = props.section_id;
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
            section_id: section_id_for_pointerdown,
        };
        dragging_source.set(Some(drag_source));
        drop_target.set(None);
        let follower = InventoryDragFollower {
            section_id: section_id_for_pointerdown,
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
        let section_id_for_flush = section_id_for_pointermove;
        let closure: DragRafClosure = Closure::new(move |_timestamp: f64| {
            DRAG_RAF_HANDLE.with(|cell| cell.set(None));
            let Some(point) = LATEST_DRAG_MOVE.with(|cell| cell.take()) else {
                return;
            };
            point.flush(drop_target, drag_follower, section_id_for_flush);
        });
        if let Ok(handle) = window.request_animation_frame(closure.as_ref().unchecked_ref()) {
            DRAG_RAF_HANDLE.with(|cell| cell.set(Some(handle)));
        }
        DRAG_RAF_CLOSURE.with(|cell| *cell.borrow_mut() = Some(closure));
    });
    let on_pointerup = EventHandler::new(move |_event: Event<PointerData>| {
        if let Some(final_point) = LATEST_DRAG_MOVE.with(|cell| cell.take()) {
            final_point.flush(drop_target, drag_follower, section_id_for_pointerup);
        }
        InventoryDragRaf::cancel();
        let drop_clone = *drop_target.read();
        let mut performed_swap = false;
        if let Some(target_id) = drop_clone
            && target_id != section_id_for_pointerup
        {
            custom_keys_service.swap_system_bindings(section_id_for_pointerup, target_id);
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
    InventoryDrag {
        on_pointerdown,
        on_pointermove,
        on_pointerup,
        on_pointercancel,
    }
}

fn use_inventory_editing(props: &InventoryCellProps) -> InventoryEditing {
    let custom_keys_service = use_custom_keys_service();
    let mut editing_section = props.editing_section;
    let section_id_for_click = props.section_id;
    let section_id_for_pick = props.section_id;
    let on_click = EventHandler::new(move |_event: MouseEvent| {
        if SUPPRESS_NEXT_CLICK.with(|cell: &Cell<bool>| cell.replace(false)) {
            return;
        }
        editing_section.set(Some(section_id_for_click));
    });
    let on_pick = EventHandler::new(move |code: KeyCode| {
        custom_keys_service.set_system_hotkey(section_id_for_pick, code);
        editing_section.set(None);
    });
    let on_close = EventHandler::new(move |_event: ()| editing_section.set(None));
    InventoryEditing {
        on_click,
        on_pick,
        on_close,
    }
}

/// Composes an inventory cell's state and behaviour.
pub(super) fn use_inventory_cell(props: &InventoryCellProps) -> InventoryCellModel {
    let view = InventoryCellView::resolve(props);
    let label_for_drag = view.key_label.clone();
    let drag = use_inventory_drag(props, label_for_drag);
    let editing = use_inventory_editing(props);
    let slot_id = props.section_id.value();
    InventoryCellModel {
        state: view.state,
        slot_id,
        dragging_attr: view.dragging_attr,
        slot_label: view.slot_label,
        key_label: view.key_label,
        conflict_title: view.conflict_title,
        is_conflict: view.is_conflict,
        is_editing: view.is_editing,
        current_code: view.current_code,
        picker_conflicts: view.picker_conflicts,
        on_pointerdown: drag.on_pointerdown,
        on_pointermove: drag.on_pointermove,
        on_pointerup: drag.on_pointerup,
        on_pointercancel: drag.on_pointercancel,
        on_click: editing.on_click,
        on_pick: editing.on_pick,
        on_close: editing.on_close,
    }
}
