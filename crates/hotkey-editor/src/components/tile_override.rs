use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_keybinds::CustomKeysFile;
use wasm_bindgen::JsCast;

use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::command_grid::{CommandGridSection, CommandGridSectionProps};
use crate::components::dialog_header::DialogHeader;
use crate::components::key_picker::{KeyPicker, KeyPickerCell, KeyPickerCellState};
use crate::domain::grid_layout::GridLayout;
use crate::domain::grid_slot::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};
use crate::domain::hotkey_override::HotkeyOverride;
use crate::domain::hotkey_token::HotkeyToken;
use crate::domain::inspector_detail::InspectorDetail;
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
    /// Hotkey for the upgraded-form unit that shares this button position
    /// (e.g. post-Barrage Siege Engine). Writes to the upgrade unit's own
    /// `Hotkey=` binding, not the base unit's.
    UpgradeHotkey,
}

#[component]
pub(crate) fn TileOverridePanel(
    detail: InspectorDetail,
    mut loaded_keys: Signal<Option<CustomKeysFile>>,
    grid_layout: Signal<GridLayout>,
    selected_from_research: Signal<bool>,
    selected_from_uprooted: Signal<bool>,
    mut tier_overrides: Signal<HashMap<String, usize>>,
    // Threaded from the app-level state so the off-state picker dialog
    // can drive the same `DragFollowerOverlay` that's already mounted at
    // the app root. Without this, dragging inside the picker hides the
    // source cell but never paints the floating follower.
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_cell: Signal<Option<DropTargetCell>>,
    drag_follower: Signal<Option<DragFollower>>,
    active_container_slots: Rc<[GridSlotId]>,
) -> Element {
    let _ = selected_from_uprooted;
    let mut editing_target = use_signal::<Option<OverrideEditTarget>>(|| None);
    // True while the player has the alt-state mini grid open. Distinct
    // signal from `editing_target` because the position picker is a modal
    // overlay rather than a hotkey picker, but only one of the two should
    // be active at a time.
    let mut alt_position_picker_open = use_signal::<bool>(|| false);
    let mut upgrade_position_picker_open = use_signal::<bool>(|| false);
    let layout_snapshot = *grid_layout.read();
    let object_id_for_capture = detail.object_id().to_string();
    let is_command_for_capture = detail.is_command();
    let is_off_state_for_capture = detail.is_off_state();
    let upgrade_unit_id_for_capture: Option<String> = detail.upgrade_unit_id().map(str::to_owned);
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
    let show_research_field = !detail.is_command() && is_research_context && !detail.info_only();
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
        Some(OverrideEditTarget::UpgradeHotkey) => detail.upgrade_hotkey_token(),
        None => None,
    };
    let picker_effective_object_id =
        if matches!(picker_target, Some(OverrideEditTarget::UpgradeHotkey)) {
            upgrade_unit_id_for_capture
                .as_deref()
                .unwrap_or(&object_id_for_capture)
                .to_string()
        } else {
            object_id_for_capture.clone()
        };
    let picker_rows: Vec<Vec<KeyPickerCell>> = if picker_open {
        build_picker_rows(
            layout_snapshot,
            &active_container_slots,
            &picker_effective_object_id,
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
    let picker_object_id = picker_effective_object_id.clone();

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
                if is_off_state_for_capture {
                    HotkeyOverride::apply_unhotkey(
                        &mut loaded_keys,
                        &picker_object_id,
                        Some(token),
                    );
                } else {
                    HotkeyOverride::apply(
                        &mut loaded_keys,
                        &picker_object_id,
                        is_command_for_capture,
                        Some(token),
                    );
                }
            }
            OverrideEditTarget::ResearchHotkey => {
                HotkeyOverride::apply_research(&mut loaded_keys, &picker_object_id, Some(token));
            }
            OverrideEditTarget::AltHotkey => {
                HotkeyOverride::apply_unhotkey(&mut loaded_keys, &picker_object_id, Some(token));
            }
            OverrideEditTarget::UpgradeHotkey => {
                HotkeyOverride::apply(&mut loaded_keys, &picker_object_id, false, Some(token));
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
                } else if detail.info_only() {
                    p { class: "tile-override-info-only", "Passive racial ability" }
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
                let show_alt_controls = has_alt_state && !is_research_context && !detail.is_command();
                rsx! {
                    if has_alt_state {
                        div { class: "tile-override-alt-state",
                            // Header mirrors the primary `tile-override-header`
                            // CSS grid (1fr | auto for hotkey | auto for the
                            // position button). Same column tracks → the V
                            // hotkey buttons in the primary and alt blocks
                            // visually line up at the same X-pixel offset.
                            div { class: "tile-override-alt-state-header",
                                div { class: "tile-override-alt-state-header-text",
                                    if let Some(alt_name) = alt_name_text {
                                        p { class: "tile-override-alt-state-label", "{alt_name}" }
                                    }
                                }
                                if show_alt_controls {
                                    button {
                                        class: "tile-override-alt-state-position-button",
                                        r#type: "button",
                                        title: "Pick where the off-state button appears on the command card",
                                        aria_label: "Edit off-state button position",
                                        onclick: move |_| {
                                            alt_position_picker_open.set(true);
                                        },
                                        // Crosshair-style position icon. Matches the
                                        // gold accent the rest of the override card uses.
                                        svg {
                                            class: "tile-override-alt-state-position-icon",
                                            view_box: "0 0 24 24",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            // Outer ring + cross-hair lines.
                                            circle { cx: "12", cy: "12", r: "5", fill: "none", stroke: "currentColor", stroke_width: "1.6" }
                                            line { x1: "12", y1: "2.5", x2: "12", y2: "6", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                                            line { x1: "12", y1: "18", x2: "12", y2: "21.5", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                                            line { x1: "2.5", y1: "12", x2: "6", y2: "12", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                                            line { x1: "18", y1: "12", x2: "21.5", y2: "12", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                                            circle { cx: "12", cy: "12", r: "1.4", fill: "currentColor" }
                                        }
                                    }
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
                        }
                    }
                }
            }
            {
                let upgrade_id = detail.upgrade_unit_id().map(str::to_owned);
                let upgrade_hotkey_token = detail.upgrade_hotkey_token();
                let upgrade_hotkey_display = upgrade_hotkey_token
                    .map(|token| token.display_label())
                    .unwrap_or_default();
                let upgrade_is_editing =
                    editing_snapshot == Some(OverrideEditTarget::UpgradeHotkey);
                let upgrade_cell_class = if upgrade_is_editing {
                    "override-key-cell editing"
                } else {
                    "override-key-cell"
                };
                let upgrade_hotkey_label = if upgrade_hotkey_display.is_empty() {
                    String::from("\u{2013}")
                } else {
                    upgrade_hotkey_display.clone()
                };
                let upgrade_hotkey_is_special = upgrade_hotkey_token
                    .map(|token| char::try_from(token).is_err())
                    .unwrap_or(false);
                let upgrade_special_flag = if upgrade_hotkey_is_special { "true" } else { "false" };
                rsx! {
                    if upgrade_id.is_some() && !is_research_context {
                        div { class: "tile-override-alt-state",
                            div { class: "tile-override-alt-state-header",
                                div { class: "tile-override-alt-state-header-text",
                                    p { class: "tile-override-alt-state-label", "Upgraded form" }
                                }
                                button {
                                    class: "tile-override-alt-state-position-button",
                                    r#type: "button",
                                    title: "Pick where the upgraded-form button appears on the command card",
                                    aria_label: "Edit upgraded-form button position",
                                    onclick: move |_| {
                                        upgrade_position_picker_open.set(true);
                                    },
                                    svg {
                                        class: "tile-override-alt-state-position-icon",
                                        view_box: "0 0 24 24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        circle { cx: "12", cy: "12", r: "5", fill: "none", stroke: "currentColor", stroke_width: "1.6" }
                                        line { x1: "12", y1: "2.5", x2: "12", y2: "6", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                                        line { x1: "12", y1: "18", x2: "12", y2: "21.5", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                                        line { x1: "2.5", y1: "12", x2: "6", y2: "12", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                                        line { x1: "18", y1: "12", x2: "21.5", y2: "12", stroke: "currentColor", stroke_width: "1.6", stroke_linecap: "round" }
                                        circle { cx: "12", cy: "12", r: "1.4", fill: "currentColor" }
                                    }
                                }
                                button {
                                    class: "{upgrade_cell_class}",
                                    "data-special": "{upgrade_special_flag}",
                                    title: "Hotkey for the upgraded form",
                                    onclick: move |_| {
                                        editing_target.set(Some(OverrideEditTarget::UpgradeHotkey));
                                    },
                                    "{upgrade_hotkey_label}"
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
            let alt_display_name = detail
                .alt_display_name()
                .map(str::to_owned)
                .unwrap_or_else(|| detail.display_name().to_string());
            // Slot list for the picker grid: the toggle's off half on top
            // (so cell_for_position picks it first when both halves
            // resolve to the same cell), then everything else from the
            // host's primary command card. We deliberately drop the
            // toggle's own on-state slot from the list so the picker grid
            // shows the off icon at the on/off position, leaving every
            // other unit slot in place for context.
            let picker_slots: Rc<[GridSlotId]> = if alt_picker_visible {
                let mut combined: Vec<GridSlotId> =
                    Vec::with_capacity(active_container_slots.len() + 1);
                combined.push(GridSlotId::ability_off(alt_picker_object_id.clone()));
                for slot in active_container_slots.iter() {
                    if let GridSlotId::Ability(ability_id) = slot
                        && ability_id.eq_ignore_ascii_case(&alt_picker_object_id)
                    {
                        continue;
                    }
                    combined.push(slot.clone());
                }
                combined.into()
            } else {
                Rc::from([] as [GridSlotId; 0])
            };
            rsx! {
                if alt_picker_visible {
                    AltPositionPicker {
                        object_id: alt_picker_object_id,
                        display_name: alt_display_name,
                        picker_slots,
                        loaded_keys,
                        grid_layout,
                        dragging_slot,
                        drop_target_cell,
                        drag_follower,
                        alt_position_picker_open,
                    }
                }
            }
        }
        {
            let upgrade_picker_visible = *upgrade_position_picker_open.read();
            let upgrade_display_name = detail
                .upgrade_display_name()
                .map(str::to_owned)
                .unwrap_or_else(|| String::from("Upgraded form"));
            // Slot list: upgraded unit first (so cell_for_position picks it),
            // then the rest of the command card minus the base unit (which
            // shares the same default position and would cause a visual
            // collision at that cell).
            let base_unit_id_for_filter = object_id_for_capture.clone();
            let upgrade_picker_id = upgrade_unit_id_for_capture.clone().unwrap_or_default();
            let upgrade_picker_slots: Rc<[GridSlotId]> = if upgrade_picker_visible && !upgrade_picker_id.is_empty() {
                let mut combined: Vec<GridSlotId> =
                    Vec::with_capacity(active_container_slots.len() + 1);
                combined.push(GridSlotId::ability(upgrade_picker_id.clone()));
                for slot in active_container_slots.iter() {
                    if let GridSlotId::Ability(base_id) = slot
                        && base_id.eq_ignore_ascii_case(&base_unit_id_for_filter)
                    {
                        continue;
                    }
                    combined.push(slot.clone());
                }
                combined.into()
            } else {
                Rc::from([] as [GridSlotId; 0])
            };
            rsx! {
                if upgrade_picker_visible && !upgrade_picker_id.is_empty() {
                    UpgradePositionPicker {
                        upgrade_unit_id: upgrade_picker_id,
                        display_name: upgrade_display_name,
                        picker_slots: upgrade_picker_slots,
                        loaded_keys,
                        grid_layout,
                        dragging_slot,
                        drop_target_cell,
                        drag_follower,
                        upgrade_position_picker_open,
                    }
                }
            }
        }
    }
}

#[component]
fn AltPositionPicker(
    object_id: String,
    display_name: String,
    picker_slots: Rc<[GridSlotId]>,
    loaded_keys: Signal<Option<CustomKeysFile>>,
    grid_layout: Signal<GridLayout>,
    // Reuse the app-level drag signals so the existing
    // `DragFollowerOverlay` (mounted at the app root) renders the
    // floating tile while the player drags inside this dialog.
    // Picker-local signals would never bind to that overlay and the
    // icon would silently disappear mid-drag.
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_cell: Signal<Option<DropTargetCell>>,
    drag_follower: Signal<Option<DragFollower>>,
    mut alt_position_picker_open: Signal<bool>,
) -> Element {
    // Selection / tier signals stay local — they only drive look inside
    // this dialog.
    let picker_selected_slot =
        use_signal::<Option<GridSlotId>>(|| Some(GridSlotId::ability_off(&object_id)));
    let picker_selected_research = use_signal::<bool>(|| false);
    let picker_selected_uprooted = use_signal::<bool>(|| false);
    let picker_tier_overrides = use_signal::<HashMap<String, usize>>(HashMap::new);
    let dialog_title = format!("Position: {display_name}");
    let restrict_draggable: Vec<GridSlotId> = vec![GridSlotId::ability_off(&object_id)];
    let _ = object_id;
    let grid_props = CommandGridSectionProps {
        heading: "Off-state position",
        slot_ids: picker_slots,
        loaded_keys,
        selected_slot: picker_selected_slot,
        selected_from_research: picker_selected_research,
        selected_from_uprooted: picker_selected_uprooted,
        tier_overrides: picker_tier_overrides,
        dragging_slot,
        drop_target_cell,
        drag_follower,
        grid_layout,
        is_research_grid: false,
        is_uprooted_grid: false,
        prevent_swap_on_drop: true,
        restrict_draggable_to: restrict_draggable,
        host_unit_id: String::new(),
    };
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: alt_position_picker_open(),
            on_open_change: move |is_open| alt_position_picker_open.set(is_open),
            DialogContent { class: "dialog-shell wc3-dialog alt-position-picker-shell".to_string(),
                DialogHeader {
                    title: dialog_title,
                    on_close: move |_| alt_position_picker_open.set(false),
                }
                div { class: "wc3-dialog-body alt-position-picker-body",
                    // Same gold-uppercase-serif treatment as
                    // `templates-dialog-explainer` and `preview-dialog-hint`
                    // so every dialog-tier explanation reads as a sibling.
                    p { class: "alt-position-picker-explainer",
                        "Drag the off-state button to a different cell. Cells holding another ability are protected; drops on top of them are rejected so the unit's primary layout stays intact."
                    }
                    // Centring wrapper — `.grid-tiles` is fixed-width
                    // (4 × 10rem) and would otherwise flush to the dialog's
                    // left edge.
                    div { class: "alt-position-picker-grid-anchor",
                        CommandGridSection { ..grid_props }
                    }
                }
            }
        }
    }
}

#[component]
fn UpgradePositionPicker(
    upgrade_unit_id: String,
    display_name: String,
    picker_slots: Rc<[GridSlotId]>,
    loaded_keys: Signal<Option<CustomKeysFile>>,
    grid_layout: Signal<GridLayout>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_cell: Signal<Option<DropTargetCell>>,
    drag_follower: Signal<Option<DragFollower>>,
    mut upgrade_position_picker_open: Signal<bool>,
) -> Element {
    let picker_selected_slot =
        use_signal::<Option<GridSlotId>>(|| Some(GridSlotId::ability(&upgrade_unit_id)));
    let picker_selected_research = use_signal::<bool>(|| false);
    let picker_selected_uprooted = use_signal::<bool>(|| false);
    let picker_tier_overrides = use_signal::<HashMap<String, usize>>(HashMap::new);
    let dialog_title = format!("Position: {display_name} (upgraded)");
    let restrict_draggable: Vec<GridSlotId> = vec![GridSlotId::ability(&upgrade_unit_id)];
    let _ = upgrade_unit_id;
    let grid_props = CommandGridSectionProps {
        heading: "Upgraded-form position",
        slot_ids: picker_slots,
        loaded_keys,
        selected_slot: picker_selected_slot,
        selected_from_research: picker_selected_research,
        selected_from_uprooted: picker_selected_uprooted,
        tier_overrides: picker_tier_overrides,
        dragging_slot,
        drop_target_cell,
        drag_follower,
        grid_layout,
        is_research_grid: false,
        is_uprooted_grid: false,
        prevent_swap_on_drop: true,
        restrict_draggable_to: restrict_draggable,
        host_unit_id: String::new(),
    };
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: upgrade_position_picker_open(),
            on_open_change: move |is_open| upgrade_position_picker_open.set(is_open),
            DialogContent { class: "dialog-shell wc3-dialog alt-position-picker-shell".to_string(),
                DialogHeader {
                    title: dialog_title,
                    on_close: move |_| upgrade_position_picker_open.set(false),
                }
                div { class: "wc3-dialog-body alt-position-picker-body",
                    p { class: "alt-position-picker-explainer",
                        "Drag the upgraded-form button to a different cell. Cells holding another ability are protected; drops on top of them are rejected so the unit's primary layout stays intact."
                    }
                    div { class: "alt-position-picker-grid-anchor",
                        CommandGridSection { ..grid_props }
                    }
                }
            }
        }
    }
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
