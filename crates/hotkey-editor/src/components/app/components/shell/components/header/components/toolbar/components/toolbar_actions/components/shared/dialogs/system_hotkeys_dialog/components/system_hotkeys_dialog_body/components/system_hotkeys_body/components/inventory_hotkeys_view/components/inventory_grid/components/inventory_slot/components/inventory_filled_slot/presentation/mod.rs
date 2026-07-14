use super::model::InventoryFilledSlotModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::{
    DID_DRAG_MOVE, DRAG_MOVEMENT_THRESHOLD_PIXELS, DRAG_ORIGIN, DRAG_RAF_CLOSURE, DRAG_RAF_HANDLE,
    DragMovePoint, DragOrigin, DragRafClosure, InventoryDragFollower, InventoryDragSource,
    LATEST_DRAG_MOVE, PENDING_INVENTORY_DRAG, PendingInventoryDrag, SUPPRESS_NEXT_CLICK,
    cancel_drag_raf,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot::SystemSlotState;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::state::use_system_hotkeys_dialog_state;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::customkeys::queries::slot_binding_query::SlotBindingView;
use dioxus::html::input_data::MouseButton;
use dioxus::html::point_interaction::PointerInteraction;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::Cell;
use std::collections::HashMap;
use warcraft_api::InventorySlots;
use warcraft_keybinds::{KeyCode, WarcraftObjectId};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(super) struct InventoryFilledSlotView {
    pub(super) state: SystemSlotState,
    pub(super) dragging: bool,
    pub(super) slot_label: String,
    pub(super) key_label: String,
    pub(super) conflict_title: String,
    pub(super) is_conflict: bool,
    pub(super) is_editing: bool,
    pub(super) current_code: KeyCode,
    pub(super) picker_conflicts: HashMap<KeyCode, Vec<String>>,
}

pub(super) struct InventoryFilledSlotInputs<'a> {
    pub(super) props: &'a InventoryFilledSlotModel,
    pub(super) binding: &'a SlotBindingView,
    pub(super) editing_section: Signal<Option<WarcraftObjectId>>,
}

impl From<InventoryFilledSlotInputs<'_>> for InventoryFilledSlotView {
    fn from(inputs: InventoryFilledSlotInputs<'_>) -> Self {
        let InventoryFilledSlotInputs {
            props,
            binding,
            editing_section,
        } = inputs;
        let dragging_source = props.dragging_source;
        let drop_target = props.drop_target;
        let section_id = props.section_id;
        let slot_index = props.slot_index;
        let is_conflict = binding.is_conflict();
        let conflict_title = if is_conflict {
            let joined_names = binding.colliding_names().join(", ");
            format!("Also used by {joined_names}")
        } else {
            String::new()
        };
        let picker_conflicts = binding.picker_conflicts().clone();
        let is_editing = *editing_section.read() == Some(section_id);
        let is_being_dragged = dragging_source
            .read()
            .as_ref()
            .map(|source| source.section_id == section_id)
            .unwrap_or(false);
        let is_drop_target = *drop_target.read() == Some(section_id);
        let state = if is_conflict {
            SystemSlotState::Conflict
        } else if is_editing || is_drop_target {
            SystemSlotState::Highlighted
        } else {
            SystemSlotState::Idle
        };
        let dragging = is_being_dragged;
        let key_label = if is_editing {
            String::from("…")
        } else {
            binding.effective_label().to_string()
        };
        let slot_label = InventorySlots::ALL
            .iter()
            .nth(slot_index)
            .map(|slot| slot.label().to_string())
            .unwrap_or_default();
        let current_code = binding.current_code();
        Self {
            state,
            dragging,
            slot_label,
            key_label,
            conflict_title,
            is_conflict,
            is_editing,
            current_code,
            picker_conflicts,
        }
    }
}

pub(super) struct InventoryFilledSlotPresentation {
    pub(super) state: SystemSlotState,
    pub(super) dragging: bool,
    pub(super) slot_label: String,
    pub(super) key_label: String,
    pub(super) tooltip_text: String,
    pub(super) tooltip_placement: TooltipPlacement,
    pub(super) conflict: bool,
    pub(super) is_editing: bool,
    pub(super) title: String,
    pub(super) current_code: KeyCode,
    pub(super) conflicts: HashMap<KeyCode, Vec<String>>,
    pub(super) open: bool,
    pub(super) on_pointerdown: EventHandler<Event<PointerData>>,
    pub(super) on_pointermove: EventHandler<Event<PointerData>>,
    pub(super) on_pointerup: EventHandler<Event<PointerData>>,
    pub(super) on_pointercancel: EventHandler<Event<PointerData>>,
    pub(super) on_click: EventHandler<MouseEvent>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) on_close: EventHandler<()>,
}

struct InventoryDrag {
    on_pointerdown: EventHandler<Event<PointerData>>,
    on_pointermove: EventHandler<Event<PointerData>>,
    on_pointerup: EventHandler<Event<PointerData>>,
    on_pointercancel: EventHandler<Event<PointerData>>,
}

struct InventoryEditing {
    on_click: EventHandler<MouseEvent>,
    on_pick: EventHandler<KeyCode>,
    on_close: EventHandler<()>,
}

fn use_inventory_drag(props: &InventoryFilledSlotModel, label_for_drag: String) -> InventoryDrag {
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
        let cell_lookup = target_element.closest(".inventory-filled-slot");
        let Ok(Some(cell_element)) = cell_lookup else {
            return;
        };
        let cell_rect = cell_element.get_bounding_client_rect();
        let cursor_horizontal_position = f64::from(web_event.client_x());
        let cursor_vertical_position = f64::from(web_event.client_y());
        let click_offset_horizontal = cursor_horizontal_position - cell_rect.left();
        let click_offset_vertical = cursor_vertical_position - cell_rect.top();
        let pointer_id = web_event.pointer_id();
        let drag_origin = DragOrigin {
            cursor_horizontal_position,
            cursor_vertical_position,
        };
        DRAG_ORIGIN.with(|cell| cell.set(Some(drag_origin)));
        DID_DRAG_MOVE.with(|cell: &Cell<bool>| cell.set(false));
        let pending = PendingInventoryDrag {
            section_id: section_id_for_pointerdown,
            label: label_for_drag.clone(),
            click_offset_horizontal,
            click_offset_vertical,
            width: cell_rect.width(),
            height: cell_rect.height(),
            cell_element,
            pointer_id,
        };
        PENDING_INVENTORY_DRAG.with(|cell| *cell.borrow_mut() = Some(pending));
    });
    let on_pointermove = EventHandler::new(move |event: Event<PointerData>| {
        let drag_is_active = dragging_source.read().is_some();
        let has_pending = PENDING_INVENTORY_DRAG.with(|cell| cell.borrow().is_some());
        if !drag_is_active && !has_pending {
            return;
        }
        let Some(web_event) = event.data().try_as_web_event() else {
            return;
        };
        let cursor_horizontal_position = f64::from(web_event.client_x());
        let cursor_vertical_position = f64::from(web_event.client_y());
        if !drag_is_active {
            let Some(origin) = DRAG_ORIGIN.with(|cell| cell.get()) else {
                return;
            };
            let horizontal_delta = cursor_horizontal_position - origin.cursor_horizontal_position;
            let vertical_delta = cursor_vertical_position - origin.cursor_vertical_position;
            let distance_squared =
                horizontal_delta * horizontal_delta + vertical_delta * vertical_delta;
            let threshold_squared = DRAG_MOVEMENT_THRESHOLD_PIXELS * DRAG_MOVEMENT_THRESHOLD_PIXELS;
            if distance_squared <= threshold_squared {
                return;
            }
            let Some(pending) = PENDING_INVENTORY_DRAG.with(|cell| cell.borrow_mut().take()) else {
                return;
            };
            if pending
                .cell_element
                .set_pointer_capture(pending.pointer_id)
                .is_err()
            {
                DRAG_ORIGIN.with(|cell| cell.set(None));
                return;
            }
            DID_DRAG_MOVE.with(|cell: &Cell<bool>| cell.set(true));
            let drag_source = InventoryDragSource {
                section_id: pending.section_id,
            };
            dragging_source.set(Some(drag_source));
            drop_target.set(None);
            let follower = InventoryDragFollower {
                section_id: pending.section_id,
                label: pending.label,
                click_offset_horizontal: pending.click_offset_horizontal,
                click_offset_vertical: pending.click_offset_vertical,
                cursor_horizontal_position,
                cursor_vertical_position,
                width: pending.width,
                height: pending.height,
            };
            drag_follower.set(Some(follower));
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
        cancel_drag_raf();
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
        PENDING_INVENTORY_DRAG.with(|cell| *cell.borrow_mut() = None);
        if did_move || performed_swap {
            SUPPRESS_NEXT_CLICK.with(|cell: &Cell<bool>| cell.set(true));
        }
        dragging_source.set(None);
        drop_target.set(None);
        drag_follower.set(None);
    });
    let on_pointercancel = EventHandler::new(move |_event: Event<PointerData>| {
        cancel_drag_raf();
        DID_DRAG_MOVE.with(|cell: &Cell<bool>| cell.set(false));
        DRAG_ORIGIN.with(|cell| cell.set(None));
        PENDING_INVENTORY_DRAG.with(|cell| *cell.borrow_mut() = None);
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

fn use_inventory_editing(
    props: &InventoryFilledSlotModel,
    editing_section: Signal<Option<WarcraftObjectId>>,
) -> InventoryEditing {
    let custom_keys_service = use_custom_keys_service();
    let mut editing_section = editing_section;
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

pub(super) fn use_inventory_filled_slot(
    props: &InventoryFilledSlotModel,
) -> InventoryFilledSlotPresentation {
    let custom_keys_service = use_custom_keys_service();
    let dialog_state = use_system_hotkeys_dialog_state();
    let editing_section = dialog_state.editing_section();
    let binding = custom_keys_service.slot_binding(props.section_id);
    let view_inputs = InventoryFilledSlotInputs {
        props,
        binding: &binding,
        editing_section,
    };
    let view = InventoryFilledSlotView::from(view_inputs);
    let label_for_drag = view.key_label.clone();
    let drag = use_inventory_drag(props, label_for_drag);
    let editing = use_inventory_editing(props, editing_section);
    let tooltip_placement = TooltipPlacement::Above;
    let title = String::from("Pick a hotkey");
    let open = true;
    InventoryFilledSlotPresentation {
        state: view.state,
        dragging: view.dragging,
        slot_label: view.slot_label,
        key_label: view.key_label,
        tooltip_text: view.conflict_title,
        tooltip_placement,
        conflict: view.is_conflict,
        is_editing: view.is_editing,
        title,
        current_code: view.current_code,
        conflicts: view.picker_conflicts,
        open,
        on_pointerdown: drag.on_pointerdown,
        on_pointermove: drag.on_pointermove,
        on_pointerup: drag.on_pointerup,
        on_pointercancel: drag.on_pointercancel,
        on_click: editing.on_click,
        on_pick: editing.on_pick,
        on_close: editing.on_close,
    }
}

impl ddd::Presentation for InventoryDrag {
    type Model = InventoryFilledSlotModel;
}
