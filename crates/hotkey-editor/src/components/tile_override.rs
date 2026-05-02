use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_keybinds::CustomKeysFile;
use wasm_bindgen::JsCast;

use crate::components::key_picker::{KeyPicker, KeyPickerCell, KeyPickerCellState};
use crate::domain::grid_layout::GridLayout;
use crate::domain::grid_slot::GridSlotId;
use crate::domain::hotkey_override::HotkeyOverride;
use crate::domain::inspector_detail::InspectorDetail;
use crate::text::description::Description;
use crate::text::tip::Tip;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OverrideEditTarget {
    Hotkey,
    ResearchHotkey,
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
    let layout_snapshot = *grid_layout.read();
    let object_id_for_capture = detail.object_id().to_string();
    let is_command_for_capture = detail.is_command();
    let layout_derived_hotkey = detail
        .button_position()
        .and_then(|position| layout_snapshot.letter_at(position.column(), position.row()))
        .map(|letter| letter.to_string());
    let layout_derived_research = detail
        .research_button_position()
        .or(detail.button_position())
        .and_then(|position| layout_snapshot.letter_at(position.column(), position.row()))
        .map(|letter| letter.to_string());
    let hotkey_display = detail
        .hotkey_letter()
        .map(String::from)
        .or(layout_derived_hotkey)
        .unwrap_or_default();
    let research_hotkey_display = detail
        .research_hotkey_letter()
        .map(String::from)
        .or(layout_derived_research)
        .unwrap_or_default();
    let is_research_context = *selected_from_research.read();
    let show_hotkey_field = !detail.is_passive() && !is_research_context;
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
    let picker_current_letter: Option<char> = match picker_target {
        Some(OverrideEditTarget::Hotkey) => hotkey_display.chars().next(),
        Some(OverrideEditTarget::ResearchHotkey) => research_hotkey_display.chars().next(),
        None => None,
    };
    let picker_rows: Vec<Vec<KeyPickerCell>> = if picker_open {
        build_picker_rows(
            layout_snapshot,
            &active_container_slots,
            &object_id_for_capture,
            picker_current_letter,
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

    let on_pick = move |letter: char| {
        let Some(active_target) = *editing_target.read() else {
            return;
        };
        let upper = letter.to_ascii_uppercase();
        let layout_snapshot_for_check = *grid_layout.read();
        let is_research_check = matches!(active_target, OverrideEditTarget::ResearchHotkey);
        let read_guard = loaded_keys.read();
        let custom_keys_ref = read_guard.as_ref();
        let conflict = HotkeyOverride::detect_conflict(
            &picker_active_container,
            &picker_object_id,
            upper,
            custom_keys_ref,
            layout_snapshot_for_check,
            is_research_check,
        );
        drop(read_guard);
        if conflict.is_some() {
            // Visual cells already convey conflicts; silently reject so the
            // keyboard fallback can't bypass the disabled state.
            return;
        }
        let new_letter = upper.to_string();
        match active_target {
            OverrideEditTarget::Hotkey => {
                HotkeyOverride::apply(
                    &mut loaded_keys,
                    &picker_object_id,
                    is_command_for_capture,
                    Some(new_letter),
                );
            }
            OverrideEditTarget::ResearchHotkey => {
                HotkeyOverride::apply_research(
                    &mut loaded_keys,
                    &picker_object_id,
                    Some(new_letter),
                );
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
                        onclick: move |_| {
                            editing_target.set(Some(OverrideEditTarget::Hotkey));
                        },
                        "{hotkey_label}"
                    }
                } else if show_research_field {
                    button {
                        class: "{research_cell_class}",
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
    }
}

const QWERTY_ROWS: &[&[char]] = &[
    &['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P'],
    &['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L'],
    &['Z', 'X', 'C', 'V', 'B', 'N', 'M'],
];

fn build_picker_rows(
    layout: GridLayout,
    container_slots: &[GridSlotId],
    target_object_id: &str,
    current_letter: Option<char>,
    is_research_context: bool,
    custom_keys: Option<&CustomKeysFile>,
) -> Vec<Vec<KeyPickerCell>> {
    let current_upper = current_letter.map(|c| c.to_ascii_uppercase());
    QWERTY_ROWS
        .iter()
        .map(|row| {
            row.iter()
                .map(|&letter| {
                    let upper = letter.to_ascii_uppercase();
                    let state = if Some(upper) == current_upper {
                        KeyPickerCellState::Current
                    } else if let Some(conflict) = HotkeyOverride::detect_conflict(
                        container_slots,
                        target_object_id,
                        upper,
                        custom_keys,
                        layout,
                        is_research_context,
                    ) {
                        KeyPickerCellState::Conflict {
                            display_name: conflict.conflicting_display_name().to_string(),
                        }
                    } else {
                        KeyPickerCellState::Available
                    };
                    KeyPickerCell::new(upper, state)
                })
                .collect()
        })
        .collect()
}
