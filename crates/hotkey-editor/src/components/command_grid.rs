use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use dioxus::html::input_data::MouseButton;
use dioxus::html::point_interaction::PointerInteraction;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use warcraft_keybinds::CustomKeysFile;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::domain::ability_cell::AbilityCell;
use crate::domain::building_traits::BuildingTraits;
use crate::domain::grid_layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};
use crate::domain::grid_slot::{
    DragFollower, DragFollowerVisual, DraggingSlot, DropTargetCell, GridSlotId,
};
use crate::domain::icons::IconUrl;
use crate::domain::object_lookup::ObjectLookup;
use crate::domain::positions::{MoveRequest, Positions};
use crate::focus::modality::FocusModality;

const DRAG_MOVEMENT_THRESHOLD_PIXELS: f64 = 4.0;
const TOUCH_CANCEL_THRESHOLD_PIXELS: f64 = 12.0;
const LONG_PRESS_MS: i32 = 300;

#[derive(Clone, Copy)]
struct DragOrigin {
    cursor_x: f64,
    cursor_y: f64,
}

struct PendingDragData {
    source_slot: GridSlotId,
    section: &'static str,
    column: u8,
    row: u8,
    visual: DragFollowerVisual,
    click_offset_x: f64,
    click_offset_y: f64,
    tile_width: f64,
    tile_height: f64,
    tile_element: web_sys::Element,
    pointer_id: i32,
    last_cursor_x: f64,
    last_cursor_y: f64,
    is_touch: bool,
}

type TouchScrollLock = Closure<dyn FnMut(web_sys::Event)>;

thread_local! {
    /// Set on a successful drag-end so the synthetic `click` that fires after
    /// `pointerup` does not also re-select the source tile.
    static SUPPRESS_NEXT_CLICK: Cell<bool> = const { Cell::new(false) };

    /// Cursor position at `pointerdown`. Used to decide whether the user
    /// actually dragged (vs. just clicked) so we know whether to suppress the
    /// trailing click.
    static DRAG_ORIGIN: Cell<Option<DragOrigin>> = const { Cell::new(None) };

    /// Set true once the cursor has travelled past the movement threshold.
    static DID_DRAG_MOVE: Cell<bool> = const { Cell::new(false) };

    /// Drag setup data captured at `pointerdown`, not yet committed to signals.
    static PENDING_DRAG: RefCell<Option<PendingDragData>> = const { RefCell::new(None) };

    /// Set when a touch/pen `pointerdown` fires so the compatibility `mouse`
    /// `pointerdown` that browsers synthesise afterward is discarded.
    static TOUCH_STARTED: Cell<bool> = const { Cell::new(false) };

    /// ID returned by `setTimeout` for the touch long-press timer.
    static TOUCH_LONG_PRESS_TIMER_ID: Cell<Option<i32>> = const { Cell::new(None) };

    /// Non-passive `touchmove` listener installed only while a touch drag is active.
    static TOUCH_SCROLL_LOCK: RefCell<Option<TouchScrollLock>> = const { RefCell::new(None) };
}

fn cancel_touch_long_press() {
    if let Some(id) = TOUCH_LONG_PRESS_TIMER_ID.with(|c| c.replace(None))
        && let Some(window) = web_sys::window()
    {
        window.clear_timeout_with_handle(id);
    }
}

fn install_touch_scroll_lock() {
    TOUCH_SCROLL_LOCK.with(|cell| {
        if cell.borrow().is_some() {
            return;
        }
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let cb = Closure::<dyn FnMut(web_sys::Event)>::new(|event: web_sys::Event| {
            event.prevent_default();
        });
        let options = web_sys::AddEventListenerOptions::new();
        options.set_capture(true);
        options.set_passive(false);
        if document
            .add_event_listener_with_callback_and_add_event_listener_options(
                "touchmove",
                cb.as_ref().unchecked_ref(),
                &options,
            )
            .is_ok()
        {
            *cell.borrow_mut() = Some(cb);
        }
    });
}

fn remove_touch_scroll_lock() {
    let cb_option = TOUCH_SCROLL_LOCK.with(|cell| cell.borrow_mut().take());
    let Some(cb) = cb_option else {
        return;
    };
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        let _ = document.remove_event_listener_with_callback_and_bool(
            "touchmove",
            cb.as_ref().unchecked_ref(),
            true,
        );
    }
}

fn reset_drag_thread_locals() {
    cancel_touch_long_press();
    remove_touch_scroll_lock();
    TOUCH_STARTED.with(|c| c.set(false));
    DID_DRAG_MOVE.with(|c| c.set(false));
    DRAG_ORIGIN.with(|c| c.set(None));
    PENDING_DRAG.with(|c| *c.borrow_mut() = None);
}

fn tile_class(
    has_occupant: bool,
    is_selected: bool,
    drag_in_progress: bool,
    is_command: bool,
    is_being_dragged: bool,
    is_drop_target: bool,
    is_off_state_blocked: bool,
) -> String {
    let base = match (has_occupant, is_selected, drag_in_progress, is_command) {
        (true, true, _, _) => "grid-tile has-ability selected",
        (true, false, _, true) => "grid-tile has-ability is-command",
        (true, false, _, false) => "grid-tile has-ability",
        (false, _, true, _) if is_off_state_blocked => "grid-tile blocked-drop-target",
        (false, _, true, _) => "grid-tile drop-target",
        (false, _, false, _) => "grid-tile",
    };
    let mut class = base.to_string();
    if is_being_dragged {
        class.push_str(" dragging-source");
    }
    if is_drop_target {
        class.push_str(" drag-over");
    }
    class
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct CommandGridSectionProps {
    pub(crate) heading: &'static str,
    pub(crate) slot_ids: Rc<[GridSlotId]>,
    pub(crate) loaded_keys: Signal<Option<CustomKeysFile>>,
    pub(crate) selected_slot: Signal<Option<GridSlotId>>,
    pub(crate) selected_from_research: Signal<bool>,
    pub(crate) selected_from_uprooted: Signal<bool>,
    pub(crate) tier_overrides: Signal<HashMap<String, usize>>,
    pub(crate) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(crate) drop_target_cell: Signal<Option<DropTargetCell>>,
    pub(crate) drag_follower: Signal<Option<DragFollower>>,
    pub(crate) grid_layout: Signal<GridLayout>,
    #[props(default = false)]
    pub(crate) is_research_grid: bool,
    #[props(default = false)]
    pub(crate) is_uprooted_grid: bool,
    /// When true, drops onto cells already occupied by another slot are
    /// rejected outright instead of swapping. The off-state position
    /// picker uses this so dragging the toggle's off half can't displace
    /// another ability's on-state on the unit's command card.
    #[props(default = false)]
    pub(crate) prevent_swap_on_drop: bool,
    /// When non-empty, only slots whose `as_str()` matches one of these
    /// ids start a drag — other slots render in their cells but are
    /// display-only. Used by the off-state picker to keep the player from
    /// accidentally rearranging the unit's primary command card while
    /// editing one toggle's off position.
    #[props(default)]
    pub(crate) restrict_draggable_to: Vec<GridSlotId>,
    /// Unit ID of the host — used to block dragging of morph abilities on
    /// alternate-form units (e.g. Burrowed Crypt Fiend). Empty string
    /// disables the check (off-state picker, build menus without a unit).
    #[props(default)]
    pub(crate) host_unit_id: String,
}

#[component]
pub(crate) fn CommandGridSection(props: CommandGridSectionProps) -> Element {
    let toast_api = use_toast();
    let read_guard = props.loaded_keys.read();
    let custom_keys_option = read_guard.as_ref();
    let layout_snapshot = *props.grid_layout.read();
    let active_slot = props.selected_slot.read().clone();
    let active_selection_is_research = *props.selected_from_research.read();

    let mut select_slot = props.selected_slot;
    let mut select_from_research = props.selected_from_research;
    let mut select_from_uprooted = props.selected_from_uprooted;
    let tier_overrides = props.tier_overrides;
    let is_research_grid = props.is_research_grid;
    let is_uprooted_grid = props.is_uprooted_grid;
    let mut dragging_slot = props.dragging_slot;
    let mut drop_target_cell = props.drop_target_cell;
    let mut drag_follower = props.drag_follower;
    let mut keys_signal = props.loaded_keys;
    let slot_ids_cloned = props.slot_ids.clone();
    let heading_text = props.heading;
    let prevent_swap_on_drop = props.prevent_swap_on_drop;
    let restrict_draggable_to: Rc<[GridSlotId]> = props.restrict_draggable_to.clone().into();
    let host_unit_id = props.host_unit_id.clone();
    let host_is_alt_form =
        !host_unit_id.is_empty() && BuildingTraits::unit_starts_in_toggle_alt_state(&host_unit_id);

    rsx! {
        div { class: "command-section",
            h3 { class: "command-section-heading", "{heading_text}" }
            div { class: "grid-tiles",
                for row in 0..COMMAND_GRID_ROWS {
                    for column in 0..COMMAND_GRID_COLUMNS {
                        {
                            let cell_with_slot = Positions::cell_for_position(
                                &slot_ids_cloned,
                                custom_keys_option,
                                is_research_grid,
                                column,
                                row,
                            );
                            let occupant_slot: Option<GridSlotId> =
                                cell_with_slot.as_ref().map(|(slot_id, _)| slot_id.clone());
                            let cell_option: Option<&AbilityCell> =
                                cell_with_slot.as_ref().map(|(_, cell)| cell);
                            let derived_letter = layout_snapshot.letter_at(column, row);
                            let is_selected = match (&occupant_slot, active_slot.as_ref()) {
                                (Some(occupant), Some(active)) => {
                                    occupant == active
                                        && active_selection_is_research == is_research_grid
                                }
                                _ => false,
                            };
                            let is_command_cell = matches!(occupant_slot, Some(GridSlotId::Command(_)));
                            let (drag_in_progress_from_this_section, dragging_id_str) = {
                                let guard = dragging_slot.read();
                                match guard.as_ref().filter(|d| d.source_section() == heading_text) {
                                    Some(d) => (true, Some(d.slot_id().as_str().to_string())),
                                    None => (false, None),
                                }
                            };
                            let is_being_dragged = match (dragging_slot.read().as_ref(), &occupant_slot) {
                                (Some(dragging), Some(occupant)) => {
                                    dragging.slot_id() == occupant && dragging.source_section() == heading_text
                                }
                                _ => false,
                            };
                            let is_drop_target_cell = drag_in_progress_from_this_section
                                && drop_target_cell
                                    .read()
                                    .as_ref()
                                    .map(|target| {
                                        target.section() == heading_text
                                            && target.column() == column
                                            && target.row() == row
                                    })
                                    .unwrap_or(false);
                            // True when this empty cell is claimed by another
                            // ability's off-state — disallowed as a drop
                            // target so the off-state isn't silently displaced.
                            // The dragging ability itself is always exempt: it
                            // may always land on its own off-state cell.
                            let is_off_state_blocked = !is_research_grid
                                && drag_in_progress_from_this_section
                                && cell_option.is_none()
                                && slot_ids_cloned.iter().any(|slot| {
                                    let GridSlotId::Ability(ability_id) = slot else {
                                        return false;
                                    };
                                    if dragging_id_str.as_deref().is_some_and(|id| {
                                        ability_id.eq_ignore_ascii_case(id)
                                    }) {
                                        return false;
                                    }
                                    Positions::current_for_ability_off(
                                        ability_id,
                                        custom_keys_option,
                                    )
                                    .is_some_and(|off_pos| {
                                        off_pos.column() == column && off_pos.row() == row
                                    })
                                });
                            let class_name = tile_class(
                                cell_option.is_some(),
                                is_selected,
                                drag_in_progress_from_this_section,
                                is_command_cell,
                                is_being_dragged,
                                is_drop_target_cell,
                                is_off_state_blocked,
                            );
                            let occupant_for_drag = occupant_slot.clone();
                            let occupant_for_click = occupant_slot.clone();
                            let occupant_for_keydown = occupant_slot.clone();
                            let restrict_draggable_to_for_drag = Rc::clone(&restrict_draggable_to);
                            let cell_object_id_option = cell_option
                                .as_ref()
                                .map(|cell| cell.object_id().to_string());
                            let cell_tier_index = cell_object_id_option
                                .as_deref()
                                .and_then(|object_id| tier_overrides.read().get(object_id).copied())
                                .unwrap_or(0);
                            let cell_database_object = cell_object_id_option
                                .as_deref()
                                .and_then(ObjectLookup::by_id);
                            let cell_tier_name = cell_database_object
                                .and_then(|warcraft_object| {
                                    warcraft_object.names().get(cell_tier_index).copied()
                                })
                                .map(String::from);
                            let cell_tier_icon = cell_database_object
                                .and_then(|warcraft_object| {
                                    warcraft_object.icons().get(cell_tier_index).copied()
                                })
                                .map(|raw_icon| IconUrl::from_database_path(raw_icon.trim()));
                            let label_text = cell_tier_name
                                .clone()
                                .or_else(|| cell_option.as_ref().map(|cell| cell.display_name().to_string()))
                                .unwrap_or_default();
                            let icon_src_option = cell_tier_icon
                                .or_else(|| cell_option.as_ref().and_then(|cell| cell.cloned_icon_src()));
                            let binding_letter_option = cell_option.as_ref().and_then(|cell| {
                                let token = if is_research_grid {
                                    cell.binding_research_hotkey()
                                        .or_else(|| cell.binding_hotkey())
                                } else {
                                    cell.binding_hotkey()
                                };
                                token.map(|value| value.display_label())
                            });
                            let is_passive_on_command_grid = !is_research_grid
                                && cell_option
                                    .as_ref()
                                    .map(|cell| ObjectLookup::is_passive_ability(cell.object_id()))
                                    .unwrap_or(false);
                            let displayed_letter: Option<String> = binding_letter_option
                                .clone()
                                .or_else(|| derived_letter.map(|character| character.to_string()));
                            let hotkey_overlay_class = if is_passive_on_command_grid {
                                "hotkey-overlay passive"
                            } else {
                                "hotkey-overlay"
                            };
                            let icon_src_for_drag = icon_src_option.as_ref().map(|url| url.to_string());
                            let label_for_drag = label_text.clone();
                            let displayed_letter_for_drag = displayed_letter.clone();
                            let slot_ids_for_drop = slot_ids_cloned.clone();
                            let is_focusable_cell = cell_option.is_some();
                            let tabindex_value = if is_focusable_cell { "0" } else { "-1" };
                            // Tile is draggable iff there's no allow-list (the
                            // normal command card) or this tile's occupant
                            // matches one of the allowed slots (the picker
                            // dialog only lets the player grab the toggle's
                            // off half). Morph abilities on alternate-form
                            // units (Burrowed Crypt Fiend, Militia, …) are
                            // never draggable — their position is shared with
                            // the primary form's ability and can't be changed
                            // independently.
                            let is_morph_on_alt_form = host_is_alt_form
                                && occupant_slot
                                    .as_ref()
                                    .map(|slot| {
                                        ObjectLookup::morph_target_unit(slot.as_str()).is_some()
                                    })
                                    .unwrap_or(false);
                            let tile_is_draggable = !is_morph_on_alt_form
                                && (restrict_draggable_to.is_empty()
                                    || occupant_slot
                                        .as_ref()
                                        .map(|slot| {
                                            restrict_draggable_to
                                                .iter()
                                                .any(|allowed| allowed == slot)
                                        })
                                        .unwrap_or(false));
                            let draggable_attr = if tile_is_draggable { "true" } else { "false" };
                            rsx! {
                                div { class: "command-tile-wrapper",
                                    div {
                                        class: "{class_name}",
                                        tabindex: "{tabindex_value}",
                                        "data-grid-row": "{row}",
                                        "data-grid-col": "{column}",
                                        "data-grid-section": "{heading_text}",
                                        "data-draggable": "{draggable_attr}",
                                        onkeydown: move |event| {
                                            let key_value = event.data().key().to_string();
                                            if key_value == " " || key_value == "Enter" {
                                                event.prevent_default();
                                                select_slot.set(occupant_for_keydown.clone());
                                                select_from_research.set(is_research_grid);
                                                select_from_uprooted.set(is_uprooted_grid);
                                                FocusModality::after_render(".tile-override-card .override-key-cell");
                                            }
                                        },
                                        onpointerdown: move |event| {
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
                                            reset_drag_thread_locals();
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
                                            let Some(source_slot) = occupant_for_drag.clone() else {
                                                return;
                                            };
                                            // Picker mode: drag origin must
                                            // match the allow-list. Used by
                                            // the off-state picker so the
                                            // dialog only lets the player
                                            // grab the toggle's off half,
                                            // not the unit's other slots.
                                            if !restrict_draggable_to_for_drag.is_empty()
                                                && !restrict_draggable_to_for_drag
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
                                            let cursor_x = f64::from(web_event.client_x());
                                            let cursor_y = f64::from(web_event.client_y());
                                            let click_offset_x = cursor_x - tile_rect.left();
                                            let click_offset_y = cursor_y - tile_rect.top();
                                            let tile_width = tile_rect.width();
                                            let tile_height = tile_rect.height();
                                            let pointer_id = web_event.pointer_id();

                                            let drag_origin = DragOrigin { cursor_x, cursor_y };
                                            DRAG_ORIGIN.with(|cell| cell.set(Some(drag_origin)));
                                            DID_DRAG_MOVE.with(|cell| cell.set(false));

                                            let visual = DragFollowerVisual::new(
                                                icon_src_for_drag.clone(),
                                                label_for_drag.clone(),
                                                displayed_letter_for_drag.clone(),
                                                is_passive_on_command_grid,
                                                is_command_cell,
                                            );
                                            let pending = PendingDragData {
                                                source_slot,
                                                section: heading_text,
                                                column,
                                                row,
                                                visual,
                                                click_offset_x,
                                                click_offset_y,
                                                tile_width,
                                                tile_height,
                                                tile_element,
                                                pointer_id,
                                                last_cursor_x: cursor_x,
                                                last_cursor_y: cursor_y,
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
                                                    if pending.tile_element.set_pointer_capture(pending.pointer_id).is_err() {
                                                        return;
                                                    }
                                                    install_touch_scroll_lock();
                                                    DID_DRAG_MOVE.with(|c| c.set(true));
                                                    let dragging = DraggingSlot::new(pending.source_slot, pending.section);
                                                    dragging_slot_cb.set(Some(dragging));
                                                    let initial_target = DropTargetCell::new(pending.section, pending.column, pending.row);
                                                    drop_target_cell_cb.set(Some(initial_target));
                                                    let follower = DragFollower::new(
                                                        pending.visual,
                                                        pending.click_offset_x,
                                                        pending.click_offset_y,
                                                        pending.last_cursor_x,
                                                        pending.last_cursor_y,
                                                        pending.tile_width,
                                                        pending.tile_height,
                                                    );
                                                    drag_follower_cb.set(Some(follower));
                                                });
                                                if let Some(window) = web_sys::window()
                                                    && let Ok(timer_id) = window
                                                        .set_timeout_with_callback_and_timeout_and_arguments_0(
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
                                        },
                                        onpointermove: move |event| {
                                            let has_pending = PENDING_DRAG.with(|cell| cell.borrow().is_some());
                                            let drag_is_active = dragging_slot.read().is_some();
                                            if !has_pending && !drag_is_active {
                                                return;
                                            }
                                            let Some(web_event) = event.data().try_as_web_event() else {
                                                return;
                                            };
                                            let cursor_x = f64::from(web_event.client_x());
                                            let cursor_y = f64::from(web_event.client_y());

                                            if has_pending {
                                                // Reject stale pending from a previous gesture whose
                                                // pointerup fired outside a tile (pointer_id mismatch).
                                                let current_pointer_id = web_event.pointer_id();
                                                let pending_pointer_id = PENDING_DRAG.with(|cell| {
                                                    cell.borrow().as_ref().map(|p| p.pointer_id)
                                                });
                                                if pending_pointer_id != Some(current_pointer_id) {
                                                    cancel_touch_long_press();
                                                    PENDING_DRAG.with(|cell| *cell.borrow_mut() = None);
                                                    DRAG_ORIGIN.with(|cell| cell.set(None));
                                                    return;
                                                }

                                                let pending_is_touch = PENDING_DRAG.with(|cell| {
                                                    cell.borrow().as_ref().map(|p| p.is_touch).unwrap_or(false)
                                                });

                                                if pending_is_touch {
                                                    // Touch pending: cancel long-press if the finger
                                                    // drifted far enough to be a swipe.
                                                    let origin_option = DRAG_ORIGIN.with(|cell| cell.get());
                                                    if let Some(origin) = origin_option {
                                                        let dx = cursor_x - origin.cursor_x;
                                                        let dy = cursor_y - origin.cursor_y;
                                                        if dx * dx + dy * dy > TOUCH_CANCEL_THRESHOLD_PIXELS * TOUCH_CANCEL_THRESHOLD_PIXELS {
                                                            cancel_touch_long_press();
                                                            PENDING_DRAG.with(|cell| *cell.borrow_mut() = None);
                                                            DRAG_ORIGIN.with(|cell| cell.set(None));
                                                            return;
                                                        }
                                                    }
                                                    // Keep last known position fresh so the follower
                                                    // appears at the right spot when the timer fires.
                                                    PENDING_DRAG.with(|cell| {
                                                        if let Some(p) = cell.borrow_mut().as_mut() {
                                                            p.last_cursor_x = cursor_x;
                                                            p.last_cursor_y = cursor_y;
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
                                                        let delta_x = cursor_x - origin.cursor_x;
                                                        let delta_y = cursor_y - origin.cursor_y;
                                                        let distance_squared = delta_x * delta_x + delta_y * delta_y;
                                                        let threshold_squared = DRAG_MOVEMENT_THRESHOLD_PIXELS
                                                            * DRAG_MOVEMENT_THRESHOLD_PIXELS;
                                                        if distance_squared > threshold_squared {
                                                            DID_DRAG_MOVE.with(|cell| cell.set(true));
                                                            let pending_option = PENDING_DRAG.with(|cell| cell.borrow_mut().take());
                                                            if let Some(pending) = pending_option {
                                                                if pending.tile_element.set_pointer_capture(pending.pointer_id).is_err() {
                                                                    DID_DRAG_MOVE.with(|cell| cell.set(false));
                                                                    DRAG_ORIGIN.with(|cell| cell.set(None));
                                                                    return;
                                                                }
                                                                let pending_source_slot = pending.source_slot;
                                                                let pending_section = pending.section;
                                                                let pending_column = pending.column;
                                                                let pending_row = pending.row;
                                                                let pending_visual = pending.visual;
                                                                let pending_click_offset_x = pending.click_offset_x;
                                                                let pending_click_offset_y = pending.click_offset_y;
                                                                let pending_tile_width = pending.tile_width;
                                                                let pending_tile_height = pending.tile_height;
                                                                let dragging = DraggingSlot::new(pending_source_slot, pending_section);
                                                                dragging_slot.set(Some(dragging));
                                                                let initial_target = DropTargetCell::new(pending_section, pending_column, pending_row);
                                                                drop_target_cell.set(Some(initial_target));
                                                                let follower = DragFollower::new(
                                                                    pending_visual,
                                                                    pending_click_offset_x,
                                                                    pending_click_offset_y,
                                                                    cursor_x,
                                                                    cursor_y,
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
                                                current_follower.set_cursor(cursor_x, cursor_y);
                                                drag_follower.set(Some(current_follower));
                                            }

                                            let document_option = web_sys::window().and_then(|w| w.document());
                                            let Some(document) = document_option else {
                                                return;
                                            };
                                            let cursor_hit_x = cursor_x as f32;
                                            let cursor_hit_y = cursor_y as f32;
                                            let elem_under_option = document
                                                .element_from_point(cursor_hit_x, cursor_hit_y);
                                            let tile_under_option = elem_under_option
                                                .and_then(|elem| elem.closest(".grid-tile").ok().flatten());
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
                                            let Some(under_row) = row_attr
                                                .as_deref()
                                                .and_then(|raw| raw.parse::<u8>().ok())
                                            else {
                                                return;
                                            };
                                            let Some(under_column) = col_attr
                                                .as_deref()
                                                .and_then(|raw| raw.parse::<u8>().ok())
                                            else {
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
                                        },
                                        onpointerup: move |_event| {
                                            // Cancel pending long-press if the finger lifted before
                                            // the timer fired (tap → select).
                                            cancel_touch_long_press();
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
                                                        let target_occupied =
                                                            Positions::cell_for_position(
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
                                                            let GridSlotId::Ability(id) = slot
                                                            else {
                                                                return None;
                                                            };
                                                            if id.eq_ignore_ascii_case(moving_id) {
                                                                return None;
                                                            }
                                                            let off_pos =
                                                                Positions::current_for_ability_off(
                                                                    id,
                                                                    custom_keys,
                                                                )?;
                                                            if off_pos.column() == drop.column()
                                                                && off_pos.row() == drop.row()
                                                            {
                                                                Some(
                                                                    ObjectLookup::by_id(id)
                                                                        .and_then(|obj| {
                                                                            obj.names().first().copied()
                                                                        })
                                                                        .unwrap_or(id.as_str())
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
                                                            ToastOptions::new().description(
                                                                "Reassign it via the override panel first.",
                                                            ),
                                                        );
                                                        fell_back_to_source = true;
                                                    } else {
                                                    let move_request = MoveRequest::new(
                                                        layout_snapshot,
                                                        &slot_ids_for_drop,
                                                        dragging.slot_id(),
                                                        drop.column(),
                                                        drop.row(),
                                                        is_research_grid,
                                                    )
                                                    .with_prevent_swap(prevent_swap_on_drop);
                                                    Positions::move_or_swap(&mut keys_signal, move_request);
                                                    let moved_slot = dragging.slot_id().clone();
                                                    select_slot.set(Some(moved_slot));
                                                    performed_swap = true;
                                                    }
                                                } else {
                                                    fell_back_to_source = true;
                                                }
                                            }
                                            let did_move = DID_DRAG_MOVE.with(|cell| cell.replace(false));
                                            DRAG_ORIGIN.with(|cell| cell.set(None));
                                            PENDING_DRAG.with(|cell| *cell.borrow_mut() = None);
                                            remove_touch_scroll_lock();
                                            if fell_back_to_source && did_move
                                                && let Some(dragging) = dragging_clone.as_ref()
                                            {
                                                let source_slot = dragging.slot_id().clone();
                                                select_slot.set(Some(source_slot));
                                                select_from_research.set(is_research_grid);
                                                select_from_uprooted.set(is_uprooted_grid);
                                            }
                                            if did_move || performed_swap {
                                                SUPPRESS_NEXT_CLICK.with(|cell| cell.set(true));
                                            }
                                            dragging_slot.set(None);
                                            drop_target_cell.set(None);
                                            drag_follower.set(None);
                                        },
                                        onpointercancel: move |_event| {
                                            reset_drag_thread_locals();
                                            dragging_slot.set(None);
                                            drop_target_cell.set(None);
                                            drag_follower.set(None);
                                        },
                                        onlostpointercapture: move |_event| {
                                            // Safety net: fires whenever pointer capture is released
                                            // (after pointerup, pointercancel, or browser scroll
                                            // takeover). Ensures drag state never stays stuck.
                                            reset_drag_thread_locals();
                                            dragging_slot.set(None);
                                            drop_target_cell.set(None);
                                            drag_follower.set(None);
                                        },
                                        onclick: move |_| {
                                            let was_suppressed = SUPPRESS_NEXT_CLICK.with(|cell| cell.replace(false));
                                            if was_suppressed {
                                                return;
                                            }
                                            select_slot.set(occupant_for_click.clone());
                                            select_from_research.set(is_research_grid);
                                            select_from_uprooted.set(is_uprooted_grid);
                                        },
                                        if let Some(source) = icon_src_option {
                                            img {
                                                src: "{source}",
                                                alt: "{label_text}",
                                                draggable: "false",
                                                loading: "lazy",
                                                decoding: "async",
                                            }
                                        } else if cell_option.is_some() {
                                            span { class: "command-label", "{label_text}" }
                                        }
                                        if let Some(letter_text) = displayed_letter.clone() {
                                            span { class: "{hotkey_overlay_class}", "{letter_text}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
