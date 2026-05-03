use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_keybinds::CustomKeysFile;
use wasm_bindgen::JsCast;

use crate::components::key_picker::{KeyPicker, KeyPickerCell, KeyPickerCellState};
use crate::domain::grid_layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};
use crate::domain::grid_slot::GridSlotId;
use crate::domain::hotkey_override::HotkeyOverride;
use crate::domain::hotkey_token::HotkeyToken;
use crate::domain::inspector_detail::InspectorDetail;
use crate::domain::positions::Positions;
use crate::text::description::Description;
use crate::text::tip::Tip;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OverrideEditTarget {
    Hotkey,
    ResearchHotkey,
    /// Off-state hotkey of a toggle ability — Stop Defend, Unburrow,
    /// unmorph. Routes through `HotkeyOverride::apply_unhotkey`, which
    /// writes the `Unhotkey` field rather than `Hotkey`.
    AltHotkey,
}

#[component]
pub(crate) fn TileOverridePanel(
    detail: InspectorDetail,
    mut loaded_keys: Signal<Option<CustomKeysFile>>,
    grid_layout: Signal<GridLayout>,
    selected_from_research: Signal<bool>,
    selected_from_uprooted: Signal<bool>,
    mut tier_overrides: Signal<HashMap<String, usize>>,
    active_container_slots: Rc<[GridSlotId]>,
) -> Element {
    let _ = selected_from_uprooted;
    let mut editing_target = use_signal::<Option<OverrideEditTarget>>(|| None);
    // True while the player has the alt-state mini grid open. Distinct
    // signal from `editing_target` because the position picker is a modal
    // overlay rather than a hotkey picker, but only one of the two should
    // be active at a time.
    let mut alt_position_picker_open = use_signal::<bool>(|| false);
    let layout_snapshot = *grid_layout.read();
    let object_id_for_capture = detail.object_id().to_string();
    let is_command_for_capture = detail.is_command();
    let layout_derived_hotkey_token = detail
        .button_position()
        .and_then(|position| layout_snapshot.letter_at(position.column(), position.row()))
        .map(HotkeyToken::from);
    let layout_derived_research_token = detail
        .research_button_position()
        .or(detail.button_position())
        .and_then(|position| layout_snapshot.letter_at(position.column(), position.row()))
        .map(HotkeyToken::from);
    let hotkey_token_display = detail.hotkey_token().or(layout_derived_hotkey_token);
    let research_hotkey_token_display = detail
        .research_hotkey_token()
        .or(layout_derived_research_token);
    let hotkey_display = hotkey_token_display
        .map(|token| token.display_label())
        .unwrap_or_default();
    let research_hotkey_display = research_hotkey_token_display
        .map(|token| token.display_label())
        .unwrap_or_default();
    let is_research_context = *selected_from_research.read();
    // Commands (e.g. CmdCancel pinned to the learn-skills grid) have a single
    // `Hotkey=` field that applies in every context — there's no separate
    // research hotkey for a command. Surface the regular hotkey field for
    // commands even in research context so the cancel button is bindable.
    let show_hotkey_field = !detail.is_passive() && (!is_research_context || detail.is_command());
    let show_research_field = !detail.is_command() && is_research_context;
    let editing_snapshot = *editing_target.read();
    let hotkey_is_editing = editing_snapshot == Some(OverrideEditTarget::Hotkey);
    let research_is_editing = editing_snapshot == Some(OverrideEditTarget::ResearchHotkey);
    let hotkey_cell_class = if hotkey_is_editing {
        "override-key-cell editing"
    } else {
        "override-key-cell"
    };
    let research_cell_class = if research_is_editing {
        "override-key-cell editing"
    } else {
        "override-key-cell"
    };
    let hotkey_label = if hotkey_display.is_empty() {
        String::from("\u{2013}")
    } else {
        hotkey_display.clone()
    };
    let research_label = if research_hotkey_display.is_empty() {
        String::from("\u{2013}")
    } else {
        research_hotkey_display.clone()
    };
    let hotkey_is_special_token = hotkey_token_display
        .map(|token| char::try_from(token).is_err())
        .unwrap_or(false);
    let research_is_special_token = research_hotkey_token_display
        .map(|token| char::try_from(token).is_err())
        .unwrap_or(false);
    let hotkey_special_flag = if hotkey_is_special_token {
        "true"
    } else {
        "false"
    };
    let research_special_flag = if research_is_special_token {
        "true"
    } else {
        "false"
    };

    // Off-state hotkey field for toggle abilities. Surfaces the `Unhotkey`
    // value from the binding (Stop Defend's key, Unburrow's key, …) and
    // routes picks through `apply_unhotkey`. Only shown when the inspector
    // detail carries an alt display name — that's the same gate the alt
    // description block already uses, so the two appear together or not.
    let alt_hotkey_token_display = detail.alt_hotkey_token();
    let alt_hotkey_display = alt_hotkey_token_display
        .map(|token| token.display_label())
        .unwrap_or_default();
    let alt_hotkey_is_editing = editing_snapshot == Some(OverrideEditTarget::AltHotkey);
    let alt_hotkey_cell_class = if alt_hotkey_is_editing {
        "override-key-cell editing"
    } else {
        "override-key-cell"
    };
    let alt_hotkey_label = if alt_hotkey_display.is_empty() {
        String::from("\u{2013}")
    } else {
        alt_hotkey_display.clone()
    };
    let alt_hotkey_is_special_token = alt_hotkey_token_display
        .map(|token| char::try_from(token).is_err())
        .unwrap_or(false);
    let alt_hotkey_special_flag = if alt_hotkey_is_special_token {
        "true"
    } else {
        "false"
    };
    let alt_button_position_label = detail
        .alt_button_position()
        .map(|position| format!("{},{}", position.column(), position.row()));

    let total_tier_count: usize = detail
        .ubertip_levels()
        .len()
        .max(detail.name_levels().len())
        .max(detail.icon_levels_len());
    let stored_tier_index = tier_overrides
        .read()
        .get(detail.object_id())
        .copied()
        .unwrap_or(0);
    let active_tier_index = if total_tier_count <= 1 {
        0
    } else {
        stored_tier_index.min(total_tier_count - 1)
    };
    let has_multiple_tiers = total_tier_count > 1;

    let active_tier_name = if has_multiple_tiers {
        detail
            .name_levels()
            .get(active_tier_index)
            .cloned()
            .unwrap_or_else(|| detail.display_name().to_string())
    } else {
        detail.display_name().to_string()
    };

    let active_ubertip_text: Option<String> = if has_multiple_tiers {
        detail.ubertip_levels().get(active_tier_index).cloned()
    } else if is_research_context {
        detail
            .research_ubertip()
            .map(String::from)
            .or_else(|| detail.ubertip().map(String::from))
    } else {
        detail.ubertip().map(String::from)
    };
    let mut primary_description_lines: Vec<String> = active_ubertip_text
        .as_deref()
        .map(Description::lines_from)
        .unwrap_or_default();
    if primary_description_lines.is_empty() {
        let fallback_tip = if is_research_context {
            detail.research_tip().or(detail.tip())
        } else {
            detail.tip()
        };
        if let Some(text) = fallback_tip {
            primary_description_lines = Tip::lines_from(text);
        }
    }
    let tier_label_text = format!("Level {} of {}", active_tier_index + 1, total_tier_count);
    let prev_object_id_for_click = detail.object_id().to_string();
    let next_object_id_for_click = detail.object_id().to_string();
    let object_id_text = detail.object_id().to_string();

    // Auto-scroll the override card into view when its detail changes (i.e. when
    // a different tile gets selected). On phone / tablet widths the override
    // sits below the command grids, and tapping a tile near the bottom of the
    // viewport otherwise leaves the override below the fold. The viewport-width
    // gate keeps desktop selections from yanking the page mid-interaction;
    // `documentElement.clientWidth` is preferred over `Window::inner_width` so
    // the threshold matches the CSS breakpoint (which excludes scrollbars).
    let scroll_dependency = detail.object_id().to_string();
    use_effect(move || {
        let _track = &scroll_dependency;
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        let Some(document_element) = document.document_element() else {
            return;
        };
        let viewport_width = document_element.client_width();
        if viewport_width > 1024 {
            return;
        }
        let target_element_result = document
            .query_selector(".tile-override-card")
            .ok()
            .flatten();
        let Some(target_element) = target_element_result else {
            return;
        };
        let Ok(html_element) = target_element.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        // In pointer/touch mode, blur any focused element before scrolling.
        // scrollIntoView moves grid tiles off-screen; without this the browser
        // "rescues" focus from the off-screen tile to a random visible element.
        let is_keyboard_mode = document
            .body()
            .map(|body| body.has_attribute("data-kb-modality"))
            .unwrap_or(false);
        if !is_keyboard_mode
            && let Some(active_el) = document
                .active_element()
                .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
        {
            let _ = active_el.blur();
        }
        html_element.scroll_into_view_with_bool(true);
    });

    let picker_open = editing_snapshot.is_some();
    let picker_target = editing_snapshot;
    let picker_is_research_context =
        matches!(picker_target, Some(OverrideEditTarget::ResearchHotkey));
    let picker_current_token: Option<HotkeyToken> = match picker_target {
        Some(OverrideEditTarget::Hotkey) => hotkey_token_display,
        Some(OverrideEditTarget::ResearchHotkey) => research_hotkey_token_display,
        Some(OverrideEditTarget::AltHotkey) => detail.alt_hotkey_token(),
        None => None,
    };
    let picker_rows: Vec<Vec<KeyPickerCell>> = if picker_open {
        build_picker_rows(
            layout_snapshot,
            &active_container_slots,
            &object_id_for_capture,
            picker_current_token,
            picker_is_research_context,
            loaded_keys.read().as_ref(),
        )
    } else {
        Vec::new()
    };
    let picker_title = match picker_target {
        Some(OverrideEditTarget::ResearchHotkey) => String::from("Pick a research hotkey"),
        _ => String::from("Pick a hotkey"),
    };
    let picker_active_container = active_container_slots.clone();
    let picker_object_id = object_id_for_capture.clone();

    let on_pick = move |token: HotkeyToken| {
        let Some(active_target) = *editing_target.read() else {
            return;
        };
        let layout_snapshot_for_check = *grid_layout.read();
        let is_research_check = matches!(active_target, OverrideEditTarget::ResearchHotkey);
        let read_guard = loaded_keys.read();
        let custom_keys_ref = read_guard.as_ref();
        let conflict = HotkeyOverride::detect_conflict(
            &picker_active_container,
            &picker_object_id,
            token,
            custom_keys_ref,
            layout_snapshot_for_check,
            is_research_check,
        );
        drop(read_guard);
        if conflict.is_some() {
            return;
        }
        match active_target {
            OverrideEditTarget::Hotkey => {
                HotkeyOverride::apply(
                    &mut loaded_keys,
                    &picker_object_id,
                    is_command_for_capture,
                    Some(token),
                );
            }
            OverrideEditTarget::ResearchHotkey => {
                HotkeyOverride::apply_research(&mut loaded_keys, &picker_object_id, Some(token));
            }
            OverrideEditTarget::AltHotkey => {
                HotkeyOverride::apply_unhotkey(&mut loaded_keys, &picker_object_id, Some(token));
            }
        }
        editing_target.set(None);
    };

    rsx! {
        div { class: "tile-override-card",
            div { class: "tile-override-header",
                div { class: "tile-override-header-text",
                    h3 { class: "tile-override-name", "{active_tier_name}" }
                    code { class: "tile-override-id", "{object_id_text}" }
                }
                if show_hotkey_field {
                    button {
                        class: "{hotkey_cell_class}",
                        "data-special": "{hotkey_special_flag}",
                        onclick: move |_| {
                            editing_target.set(Some(OverrideEditTarget::Hotkey));
                        },
                        "{hotkey_label}"
                    }
                } else if show_research_field {
                    button {
                        class: "{research_cell_class}",
                        "data-special": "{research_special_flag}",
                        onclick: move |_| {
                            editing_target.set(Some(OverrideEditTarget::ResearchHotkey));
                        },
                        "{research_label}"
                    }
                }
            }
            if !primary_description_lines.is_empty() {
                div { class: "tile-override-description",
                    for description_line in primary_description_lines.iter() {
                        p { class: "tile-override-description-line", "{description_line}" }
                    }
                }
            }
            {
                let alt_name_text = detail.alt_display_name().map(str::to_owned);
                let alt_description_lines: Vec<String> = detail
                    .alt_ubertip()
                    .map(Description::lines_from)
                    .unwrap_or_default();
                let has_alt_state = alt_name_text.is_some() || !alt_description_lines.is_empty();
                // Only let the player edit the off-state hotkey on the
                // primary command card — research grids only have a single
                // hotkey field per ability (Hero learn-skill icons aren't
                // toggles), so the alt slot is irrelevant there.
                let show_alt_hotkey = has_alt_state && !is_research_context && !detail.is_command();
                let alt_position_text = alt_button_position_label.clone();
                rsx! {
                    if has_alt_state {
                        div { class: "tile-override-alt-state",
                            div { class: "tile-override-alt-state-header",
                                if let Some(alt_name) = alt_name_text {
                                    p { class: "tile-override-alt-state-label", "When active: {alt_name}" }
                                }
                                if show_alt_hotkey {
                                    button {
                                        class: "{alt_hotkey_cell_class}",
                                        "data-special": "{alt_hotkey_special_flag}",
                                        title: "Hotkey for the off state (writes Unhotkey)",
                                        onclick: move |_| {
                                            editing_target.set(Some(OverrideEditTarget::AltHotkey));
                                        },
                                        "{alt_hotkey_label}"
                                    }
                                }
                            }
                            for description_line in alt_description_lines.iter() {
                                p { class: "tile-override-alt-state-line", "{description_line}" }
                            }
                            if show_alt_hotkey {
                                div { class: "tile-override-alt-state-position-row",
                                    if let Some(position_label) = alt_position_text {
                                        span { class: "tile-override-alt-state-meta",
                                            "Off button position: {position_label}"
                                        }
                                    } else {
                                        span { class: "tile-override-alt-state-meta",
                                            "Off button position: (none)"
                                        }
                                    }
                                    button {
                                        class: "tile-override-alt-state-position-button",
                                        title: "Pick the off-state button position (writes Unbuttonpos)",
                                        onclick: move |_| {
                                            alt_position_picker_open.set(true);
                                        },
                                        "Edit position"
                                    }
                                }
                            } else if let Some(position_label) = alt_position_text {
                                p { class: "tile-override-alt-state-meta",
                                    "Off button position: {position_label}"
                                }
                            }
                        }
                    }
                }
            }
            if has_multiple_tiers {
                div { class: "tile-override-tier-footer",
                    button {
                        class: "tile-override-tier-button",
                        aria_label: "Previous level",
                        onclick: move |_| {
                            let tier_count = total_tier_count;
                            let id_key = prev_object_id_for_click.clone();
                            let mut writable_guard = tier_overrides.write();
                            let current = writable_guard.get(id_key.as_str()).copied().unwrap_or(0);
                            let next = if current == 0 { tier_count - 1 } else { current - 1 };
                            writable_guard.insert(id_key, next);
                        }
                    }
                    span { class: "tile-override-tier-label", "{tier_label_text}" }
                    button {
                        class: "tile-override-tier-button",
                        aria_label: "Next level",
                        onclick: move |_| {
                            let tier_count = total_tier_count;
                            let id_key = next_object_id_for_click.clone();
                            let mut writable_guard = tier_overrides.write();
                            let current = writable_guard.get(id_key.as_str()).copied().unwrap_or(0);
                            let next = (current + 1) % tier_count;
                            writable_guard.insert(id_key, next);
                        }
                    }
                }
            }
        }
        if picker_open {
            KeyPicker {
                title: picker_title,
                rows: picker_rows,
                open: true,
                on_pick,
                on_close: move |_| editing_target.set(None),
            }
        }
        {
            let alt_picker_visible = *alt_position_picker_open.read();
            let alt_picker_object_id = object_id_for_capture.clone();
            let cells = if alt_picker_visible {
                build_alt_position_cells(
                    &alt_picker_object_id,
                    detail.alt_button_position(),
                    &active_container_slots,
                    loaded_keys.read().as_ref(),
                )
            } else {
                Vec::new()
            };
            rsx! {
                if alt_picker_visible {
                    AltPositionPicker {
                        object_id: alt_picker_object_id,
                        cells,
                        loaded_keys,
                        alt_position_picker_open,
                    }
                }
            }
        }
    }
}

/// One cell in the off-state position picker.
#[derive(Clone, PartialEq, Eq)]
struct AltPositionCell {
    column: u8,
    row: u8,
    /// `Some(name)` when another ability already lives here and the cell
    /// should be blocked. `None` when the cell is either empty, hosts the
    /// host ability's *own* on-state (overlap is allowed — most toggles
    /// default to that), or hosts the host's current off-state.
    blocked_by: Option<String>,
    is_current_off: bool,
    is_own_on: bool,
}

fn build_alt_position_cells(
    self_id: &str,
    current_off_position: Option<warcraft_api::ButtonPosition>,
    container_slots: &[GridSlotId],
    custom_keys: Option<&CustomKeysFile>,
) -> Vec<AltPositionCell> {
    let mut cells: Vec<AltPositionCell> =
        Vec::with_capacity(usize::from(COMMAND_GRID_ROWS) * usize::from(COMMAND_GRID_COLUMNS));
    for row in 0..COMMAND_GRID_ROWS {
        for column in 0..COMMAND_GRID_COLUMNS {
            // Find which (if any) slot resolves to this cell on the unit's
            // primary command card. We only check `Ability` and `Command`
            // slots — the off-state picker is concerned with on-state
            // occupancy of *other* abilities, plus the host's own on-state
            // (which is allowed but flagged so the user sees the overlap).
            let occupant = Positions::cell_for_position(
                container_slots,
                custom_keys,
                false,
                column,
                row,
            );
            let (blocked_by, is_own_on) = match occupant {
                Some((slot, cell)) => {
                    if slot.as_str().eq_ignore_ascii_case(self_id)
                        && matches!(slot, GridSlotId::Ability(_))
                    {
                        (None, true)
                    } else {
                        (Some(cell.cloned_display_name()), false)
                    }
                }
                None => (None, false),
            };
            let is_current_off = current_off_position
                .map(|position| position.column() == column && position.row() == row)
                .unwrap_or(false);
            cells.push(AltPositionCell {
                column,
                row,
                blocked_by,
                is_current_off,
                is_own_on,
            });
        }
    }
    cells
}

#[component]
fn AltPositionPicker(
    object_id: String,
    cells: Vec<AltPositionCell>,
    mut loaded_keys: Signal<Option<CustomKeysFile>>,
    mut alt_position_picker_open: Signal<bool>,
) -> Element {
    let object_id_rc: Rc<str> = Rc::from(object_id.as_str());
    rsx! {
        div {
            class: "alt-position-picker-backdrop",
            onclick: move |_| alt_position_picker_open.set(false),
            div {
                class: "alt-position-picker-dialog",
                onclick: move |event| event.stop_propagation(),
                h3 { class: "alt-position-picker-title", "Off button position" }
                p { class: "alt-position-picker-hint",
                    "Pick a free cell. Greyed cells are taken by another ability; the cell with a dot is the on-state of this ability (overlap allowed)."
                }
                div { class: "alt-position-picker-grid",
                    for cell in cells.iter() {
                        {
                            let is_blocked = cell.blocked_by.is_some();
                            let is_current_off = cell.is_current_off;
                            let is_own_on = cell.is_own_on;
                            let aria_label = match (&cell.blocked_by, is_own_on, is_current_off) {
                                (Some(name), _, _) => format!("{} (occupied by {})", cell_label(cell.column, cell.row), name),
                                (None, true, _) => format!("{} (your on-state)", cell_label(cell.column, cell.row)),
                                (None, false, true) => format!("{} (current off)", cell_label(cell.column, cell.row)),
                                _ => cell_label(cell.column, cell.row),
                            };
                            let blocked_attr = if is_blocked { "true" } else { "false" };
                            let current_attr = if is_current_off { "true" } else { "false" };
                            let own_on_attr = if is_own_on { "true" } else { "false" };
                            let column = cell.column;
                            let row = cell.row;
                            let cell_object_id = Rc::clone(&object_id_rc);
                            rsx! {
                                button {
                                    class: "alt-position-picker-cell",
                                    "data-blocked": "{blocked_attr}",
                                    "data-current-off": "{current_attr}",
                                    "data-own-on": "{own_on_attr}",
                                    disabled: is_blocked,
                                    aria_label: "{aria_label}",
                                    onclick: move |_| {
                                        if is_blocked {
                                            return;
                                        }
                                        Positions::assign_off_position(
                                            &mut loaded_keys,
                                            &cell_object_id,
                                            column,
                                            row,
                                        );
                                        alt_position_picker_open.set(false);
                                    },
                                    if let Some(name) = cell.blocked_by.as_deref() {
                                        span { class: "alt-position-picker-cell-occupant", "{name}" }
                                    } else if is_own_on {
                                        span { class: "alt-position-picker-cell-marker", "•" }
                                    } else if is_current_off {
                                        span { class: "alt-position-picker-cell-marker", "✓" }
                                    }
                                }
                            }
                        }
                    }
                }
                button {
                    class: "alt-position-picker-cancel",
                    onclick: move |_| alt_position_picker_open.set(false),
                    "Cancel"
                }
            }
        }
    }
}

fn cell_label(column: u8, row: u8) -> String {
    format!("Column {}, Row {}", column, row)
}

const PICKER_ROWS: &[&[HotkeyToken]] = &[
    &[
        HotkeyToken::Letter { character: 'Q' },
        HotkeyToken::Letter { character: 'W' },
        HotkeyToken::Letter { character: 'E' },
        HotkeyToken::Letter { character: 'R' },
        HotkeyToken::Letter { character: 'T' },
        HotkeyToken::Letter { character: 'Y' },
        HotkeyToken::Letter { character: 'U' },
        HotkeyToken::Letter { character: 'I' },
        HotkeyToken::Letter { character: 'O' },
        HotkeyToken::Letter { character: 'P' },
    ],
    &[
        HotkeyToken::Letter { character: 'A' },
        HotkeyToken::Letter { character: 'S' },
        HotkeyToken::Letter { character: 'D' },
        HotkeyToken::Letter { character: 'F' },
        HotkeyToken::Letter { character: 'G' },
        HotkeyToken::Letter { character: 'H' },
        HotkeyToken::Letter { character: 'J' },
        HotkeyToken::Letter { character: 'K' },
        HotkeyToken::Letter { character: 'L' },
    ],
    &[
        HotkeyToken::Letter { character: 'Z' },
        HotkeyToken::Letter { character: 'X' },
        HotkeyToken::Letter { character: 'C' },
        HotkeyToken::Letter { character: 'V' },
        HotkeyToken::Letter { character: 'B' },
        HotkeyToken::Letter { character: 'N' },
        HotkeyToken::Letter { character: 'M' },
    ],
    &[
        HotkeyToken::Escape,
        HotkeyToken::MouseBack,
        HotkeyToken::MouseForward,
    ],
];

fn build_picker_rows(
    layout: GridLayout,
    container_slots: &[GridSlotId],
    target_object_id: &str,
    current_token: Option<HotkeyToken>,
    is_research_context: bool,
    custom_keys: Option<&CustomKeysFile>,
) -> Vec<Vec<KeyPickerCell>> {
    PICKER_ROWS
        .iter()
        .map(|row| {
            row.iter()
                .map(|token| {
                    let token_value = *token;
                    let state = if Some(token_value) == current_token {
                        KeyPickerCellState::Current
                    } else if let Some(conflict) = HotkeyOverride::detect_conflict(
                        container_slots,
                        target_object_id,
                        token_value,
                        custom_keys,
                        layout,
                        is_research_context,
                    ) {
                        let display_name = conflict.conflicting_display_name().to_string();
                        KeyPickerCellState::Conflict { display_name }
                    } else {
                        KeyPickerCellState::Available
                    };
                    KeyPickerCell::new(token_value, state)
                })
                .collect()
        })
        .collect()
}
