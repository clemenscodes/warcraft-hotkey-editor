use std::cell::Cell;

use dioxus::html::input_data::MouseButton;
use dioxus::html::point_interaction::PointerInteraction;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::CustomKeys;
use wasm_bindgen::JsCast;

use crate::components::system_hotkeys::key_picker_dialog::SystemKeyPickerDialog;
use crate::model::grid::{CursorPoint, HitTestPoint};
use warcraft_keybinds::EffectiveBinding;
use warcraft_keybinds::SystemBindingMap;

use super::{
    DID_DRAG_MOVE, DRAG_MOVEMENT_THRESHOLD_PIXELS, DRAG_ORIGIN, DragOrigin, InventoryDragFollower,
    InventoryDragSource, SUPPRESS_NEXT_CLICK,
};

#[derive(Props, Clone, PartialEq)]
pub(super) struct InventoryCellProps {
    pub(super) slot_index: usize,
    pub(super) section_id: String,
    pub(super) default_hotkey: u32,
    pub(super) default_modifier: SystemKeybindModifier,
    pub(super) loaded_keys: Signal<Option<CustomKeys>>,
    pub(super) editing_section: Signal<Option<String>>,
    pub(super) dragging_source: Signal<Option<InventoryDragSource>>,
    pub(super) drop_target: Signal<Option<String>>,
    pub(super) drag_follower: Signal<Option<InventoryDragFollower>>,
    pub(super) binding_map: ReadSignal<SystemBindingMap>,
}

#[component]
pub(super) fn InventoryCell(props: InventoryCellProps) -> Element {
    let slot_index = props.slot_index;
    let section_id = props.section_id;
    let default_hotkey = props.default_hotkey;
    let default_modifier = props.default_modifier;
    let loaded_keys = props.loaded_keys;
    let mut editing_section = props.editing_section;
    let mut dragging_source = props.dragging_source;
    let mut drop_target = props.drop_target;
    let mut drag_follower = props.drag_follower;
    let binding_map = props.binding_map;
    let mut keys_signal = loaded_keys;
    let read_guard = loaded_keys.read();
    let custom_keys_ref = read_guard.as_ref();
    let effective = EffectiveBinding::resolve_from_file(
        custom_keys_ref,
        &section_id,
        default_hotkey,
        default_modifier,
    );
    drop(read_guard);
    let map_guard = binding_map.read();
    let collisions =
        map_guard.collisions_for(&section_id, effective.hotkey_code(), effective.modifier());
    let is_in_conflict = !collisions.is_empty();
    let conflict_title = if is_in_conflict {
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
    let key_label = effective.label();
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
    let mut cell_class = String::from("wc3-slot");
    if is_editing {
        cell_class.push_str(" editing");
    }
    if is_being_dragged {
        cell_class.push_str(" dragging-source");
    }
    if is_drop_target {
        cell_class.push_str(" drag-over");
    }
    if is_in_conflict {
        cell_class.push_str(" conflict");
    }
    let section_id_for_click = section_id.clone();
    let section_id_for_pick = section_id.clone();
    let section_id_for_pointerdown = section_id.clone();
    let section_id_for_pointermove = section_id.clone();
    let section_id_for_pointerup = section_id.clone();
    let label_for_drag = key_label.clone();
    let handle_pointerdown = move |event: Event<PointerData>| {
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
        let cell_lookup = target_element.closest(".wc3-slot");
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
    };
    let handle_pointermove = move |event: Event<PointerData>| {
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
            elem_under_option.and_then(|elem| elem.closest(".wc3-slot").ok().flatten());
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
        if target_id_string == section_id_for_pointermove {
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
    };
    let handle_pointerup = move |_event| {
        let drop_clone = drop_target.read().clone();
        let mut performed_swap = false;
        if let Some(target_id) = drop_clone
            && target_id != section_id_for_pointerup
        {
            keys_signal
                .write()
                .get_or_insert_with(|| CustomKeys::from(""))
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
    };
    let handle_pointercancel = move |_| {
        DID_DRAG_MOVE.with(|cell: &Cell<bool>| cell.set(false));
        DRAG_ORIGIN.with(|cell| cell.set(None));
        dragging_source.set(None);
        drop_target.set(None);
        drag_follower.set(None);
    };
    let handle_click = move |_| {
        if SUPPRESS_NEXT_CLICK.with(|cell: &Cell<bool>| cell.replace(false)) {
            return;
        }
        editing_section.set(Some(section_id_for_click.clone()));
    };
    let handle_pick = move |code: u32| {
        let mut guard = keys_signal.write();
        let file = guard.get_or_insert_with(|| CustomKeys::from(""));
        file.set_system_hotkey(&section_id_for_pick, code);
        drop(guard);
        editing_section.set(None);
    };
    let handle_picker_close = move |_| editing_section.set(None);
    rsx! {
        div {
            class: cell_class,
            "data-inventory-slot": section_id,
            tabindex: "0",
            "data-tooltip": conflict_title,
            "data-tooltip-placement": "above",
            onpointerdown: handle_pointerdown,
            onpointermove: handle_pointermove,
            onpointerup: handle_pointerup,
            onpointercancel: handle_pointercancel,
            onclick: handle_click,
            div { class: "wc3-slot-label", "Slot {slot_index + 1}" }
            div { class: "wc3-slot-key",
                if is_editing { "…" } else { {key_label} }
            }
        }
        if is_editing {
            SystemKeyPickerDialog {
                title: String::from("Pick a hotkey"),
                current_code: effective.hotkey_code(),
                conflicts: picker_conflicts,
                open: true,
                on_pick: handle_pick,
                on_close: handle_picker_close,
            }
        }
    }
}
