use std::cell::Cell;

use dioxus::html::input_data::MouseButton;
use dioxus::html::point_interaction::PointerInteraction;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use warcraft_api::SystemKeybindModifier;
use warcraft_keybinds::{CustomKeysFile, Hotkey};
use wasm_bindgen::JsCast;

use crate::components::system_hotkeys::key_cell::EffectiveBinding;
use crate::components::system_hotkeys::key_picker_dialog::SystemKeyPickerDialog;
use crate::domain::cursor_hit::{CursorPoint, HitTestPoint};
use crate::system_hotkeys::binding_map::SystemBindingMap;
use crate::system_hotkeys::category::SystemHotkeysCategory;

const SLOT_FRAME_GOLD: Asset = asset!("/assets/webui/widgets/listitems/list-item-focus-border.png");

const DRAG_MOVEMENT_THRESHOLD_PIXELS: f64 = 4.0;
const INVENTORY_COLUMNS: usize = 2;
const INVENTORY_ROWS: usize = 3;

#[derive(Clone, Copy)]
struct DragOrigin {
    cursor_x: f64,
    cursor_y: f64,
}

thread_local! {
    static SUPPRESS_NEXT_CLICK: Cell<bool> = const { Cell::new(false) };
    static DRAG_ORIGIN: Cell<Option<DragOrigin>> = const { Cell::new(None) };
    static DID_DRAG_MOVE: Cell<bool> = const { Cell::new(false) };
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct InventoryDragFollower {
    section_id: String,
    label: String,
    click_offset_x: f64,
    click_offset_y: f64,
    cursor_x: f64,
    cursor_y: f64,
    width: f64,
    height: f64,
}

impl InventoryDragFollower {
    pub(crate) fn left(&self) -> f64 {
        self.cursor_x - self.click_offset_x
    }

    pub(crate) fn top(&self) -> f64 {
        self.cursor_y - self.click_offset_y
    }

    pub(crate) fn width(&self) -> f64 {
        self.width
    }

    pub(crate) fn height(&self) -> f64 {
        self.height
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct InventoryDragSource {
    section_id: String,
}

#[component]
pub(crate) fn InventoryHotkeysView(
    loaded_keys: Signal<Option<CustomKeysFile>>,
    editing_section: Signal<Option<String>>,
    drag_follower: Signal<Option<InventoryDragFollower>>,
) -> Element {
    let entries = SystemHotkeysCategory::Inventory.entries();
    let dragging_source = use_signal::<Option<InventoryDragSource>>(|| None);
    let drop_target = use_signal::<Option<String>>(|| None);

    let slot_frame_url = SLOT_FRAME_GOLD;
    let frame_style = format!("--wc3-slot-frame: url('{slot_frame_url}');");
    rsx! {
        div { class: "wc3-stage",
            p { class: "wc3-stage-hint",
                "Drag a slot onto another to swap their keys."
            }
            div { class: "wc3-inventory-grid", style: "{frame_style}",
                for row in 0..INVENTORY_ROWS {
                    for column in 0..INVENTORY_COLUMNS {
                        {
                            let slot_index = row * INVENTORY_COLUMNS + column;
                            let entry_option = entries.get(slot_index).copied();
                            match entry_option {
                                Some(entry) => rsx! {
                                    InventoryCell {
                                        slot_index,
                                        section_id: entry.section_id().to_string(),
                                        default_hotkey: entry.default_hotkey(),
                                        default_modifier: entry.default_modifier(),
                                        loaded_keys,
                                        editing_section,
                                        dragging_source,
                                        drop_target,
                                        drag_follower,
                                    }
                                },
                                None => rsx! {
                                    div { class: "wc3-slot empty", "—" }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InventoryCell(
    slot_index: usize,
    section_id: String,
    default_hotkey: u32,
    default_modifier: SystemKeybindModifier,
    loaded_keys: Signal<Option<CustomKeysFile>>,
    mut editing_section: Signal<Option<String>>,
    mut dragging_source: Signal<Option<InventoryDragSource>>,
    mut drop_target: Signal<Option<String>>,
    mut drag_follower: Signal<Option<InventoryDragFollower>>,
) -> Element {
    let mut keys_signal = loaded_keys;
    let read_guard = loaded_keys.read();
    let custom_keys_ref = read_guard.as_ref();
    let effective = EffectiveBinding::resolve_from_file(
        custom_keys_ref,
        &section_id,
        default_hotkey,
        default_modifier,
    );
    let binding_map = SystemBindingMap::build(custom_keys_ref);
    drop(read_guard);
    let collisions =
        binding_map.collisions_for(&section_id, effective.hotkey_code, effective.modifier);
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
    let picker_conflicts = binding_map.picker_conflicts(&section_id, effective.modifier);
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
    rsx! {
        div {
            class: "{cell_class}",
            "data-inventory-slot": "{section_id}",
            tabindex: "0",
            "data-tooltip": "{conflict_title}",
            "data-tooltip-placement": "above",
            onpointerdown: move |event| {
                if event.data().trigger_button() != Some(MouseButton::Primary) {
                    return;
                }
                let Some(web_event) = event.data().try_as_web_event() else {
                    return;
                };
                // Drag-to-swap on touch / pen collides with both tap-to-pick
                // and long-press-for-tooltip. Coarse-pointer users don't need
                // reordering here — the picker dialog still lets them pick a
                // hotkey for the slot directly. Skip the drag setup entirely
                // so a tap fires only `onclick` and a hold fires only the
                // tooltip long-press.
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
                let cursor_x = f64::from(web_event.client_x());
                let cursor_y = f64::from(web_event.client_y());
                let click_offset_x = cursor_x - cell_rect.left();
                let click_offset_y = cursor_y - cell_rect.top();
                let pointer_id = web_event.pointer_id();
                let _ = cell_element.set_pointer_capture(pointer_id);

                let drag_origin = DragOrigin { cursor_x, cursor_y };
                DRAG_ORIGIN.with(|cell| cell.set(Some(drag_origin)));
                DID_DRAG_MOVE.with(|cell| cell.set(false));

                let drag_source = InventoryDragSource { section_id: section_id_for_pointerdown.clone() };
                dragging_source.set(Some(drag_source));
                drop_target.set(None);
                let follower = InventoryDragFollower {
                    section_id: section_id_for_pointerdown.clone(),
                    label: label_for_drag.clone(),
                    click_offset_x,
                    click_offset_y,
                    cursor_x,
                    cursor_y,
                    width: cell_rect.width(),
                    height: cell_rect.height(),
                };
                drag_follower.set(Some(follower));
            },
            onpointermove: move |event| {
                if dragging_source.read().is_none() {
                    return;
                }
                let Some(web_event) = event.data().try_as_web_event() else {
                    return;
                };
                let cursor_x = f64::from(web_event.client_x());
                let cursor_y = f64::from(web_event.client_y());

                if let Some(origin) = DRAG_ORIGIN.with(|cell| cell.get()) {
                    let delta_x = cursor_x - origin.cursor_x;
                    let delta_y = cursor_y - origin.cursor_y;
                    let distance_squared = delta_x * delta_x + delta_y * delta_y;
                    let threshold_squared = DRAG_MOVEMENT_THRESHOLD_PIXELS * DRAG_MOVEMENT_THRESHOLD_PIXELS;
                    if distance_squared > threshold_squared {
                        DID_DRAG_MOVE.with(|cell| cell.set(true));
                    }
                }

                let current_follower_option = drag_follower.read().clone();
                if let Some(mut current_follower) = current_follower_option {
                    current_follower.cursor_x = cursor_x;
                    current_follower.cursor_y = cursor_y;
                    drag_follower.set(Some(current_follower));
                }

                let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                    return;
                };
                let cursor_point = CursorPoint::new(cursor_x, cursor_y);
                let hit_test_point = HitTestPoint::from(cursor_point);
                let hit_test_x = hit_test_point.x();
                let hit_test_y = hit_test_point.y();
                let elem_under_option = document.element_from_point(hit_test_x, hit_test_y);
                let cell_under_option = elem_under_option
                    .and_then(|elem| elem.closest(".wc3-slot").ok().flatten());
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
            },
            onpointerup: move |_event| {
                let drop_clone = drop_target.read().clone();
                let mut performed_swap = false;
                if let Some(target_id) = drop_clone
                    && target_id != section_id_for_pointerup
                {
                    keys_signal
                        .write()
                        .get_or_insert_with(|| CustomKeysFile::from(""))
                        .swap_system_bindings(&section_id_for_pointerup, &target_id);
                    performed_swap = true;
                }
                let did_move = DID_DRAG_MOVE.with(|cell| cell.replace(false));
                DRAG_ORIGIN.with(|cell| cell.set(None));
                if did_move || performed_swap {
                    SUPPRESS_NEXT_CLICK.with(|cell| cell.set(true));
                }
                dragging_source.set(None);
                drop_target.set(None);
                drag_follower.set(None);
            },
            onpointercancel: move |_| {
                DID_DRAG_MOVE.with(|cell| cell.set(false));
                DRAG_ORIGIN.with(|cell| cell.set(None));
                dragging_source.set(None);
                drop_target.set(None);
                drag_follower.set(None);
            },
            onclick: move |_| {
                if SUPPRESS_NEXT_CLICK.with(|cell| cell.replace(false)) {
                    return;
                }
                editing_section.set(Some(section_id_for_click.clone()));
            },
            div { class: "wc3-slot-label", "Slot {slot_index + 1}" }
            div { class: "wc3-slot-key",
                if is_editing { "…" } else { "{key_label}" }
            }
        }
        if is_editing {
            SystemKeyPickerDialog {
                title: String::from("Pick a hotkey"),
                current_code: effective.hotkey_code,
                conflicts: picker_conflicts,
                open: true,
                on_pick: move |code: u32| {
                    let mut guard = keys_signal.write();
                    let file = guard.get_or_insert_with(|| CustomKeysFile::from(""));
                    if let Some(binding) = file.system_mut(&section_id_for_pick) {
                        binding.set_hotkey(Hotkey::VirtualKey(code));
                    }
                    drop(guard);
                    editing_section.set(None);
                },
                on_close: move |_| editing_section.set(None),
            }
        }
    }
}
