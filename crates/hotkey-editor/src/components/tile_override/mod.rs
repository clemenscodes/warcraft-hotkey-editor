mod alt_state_section;
mod description;
mod key_field;
mod position_picker;
mod upgrade_section;
mod upgrade_tier;

use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::{CustomKeys, HotkeyTarget, HotkeyToken};
use wasm_bindgen::JsCast;

use crate::components::shared::key_picker::{KeyPicker, KeyPickerCell, KeyPickerCellState};
use warcraft_keybinds::InspectorDetail;
use warcraft_keybinds::text::description::Description;
use warcraft_keybinds::text::tip::Tip;

use crate::model::grid::GridLayout;
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};
use crate::services::customkeys::hotkey_override::HotkeyOverride;

use alt_state_section::AltStateSection;
use description::AbilityDescription;
use key_field::OverrideKeyField;
use position_picker::{AltPositionPicker, UpgradePositionPicker};
use upgrade_section::UpgradeSection;
use upgrade_tier::UpgradeTierSelector;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OverrideEditTarget {
    Hotkey,
    ResearchHotkey,
    /// Off-state hotkey of a toggle ability — Stop Defend, Unburrow,
    /// unmorph. Writes the `Unhotkey` field rather than `Hotkey`.
    AltHotkey,
    /// Hotkey for the upgraded-form unit that shares this button position
    /// (e.g. post-Barrage Siege Engine). Writes to the upgrade unit's own
    /// `Hotkey=` binding, not the base unit's.
    UpgradeHotkey,
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct TileOverridePanelProps {
    pub(crate) detail: InspectorDetail,
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) grid_layout: Signal<GridLayout>,
    pub(crate) selected_from_research: Signal<bool>,
    pub(crate) selected_from_uprooted: Signal<bool>,
    pub(crate) tier_overrides: Signal<HashMap<String, usize>>,
    // Threaded from the app-level state so the off-state picker dialog
    // can drive the same `DragFollowerOverlay` that's already mounted at
    // the app root. Without this, dragging inside the picker hides the
    // source cell but never paints the floating follower.
    pub(crate) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(crate) drop_target_cell: Signal<Option<DropTargetCell>>,
    pub(crate) drag_follower: Signal<Option<DragFollower>>,
    pub(crate) active_container_slots: Rc<[GridSlotId]>,
}

#[component]
pub(crate) fn TileOverridePanel(props: TileOverridePanelProps) -> Element {
    let detail = props.detail;
    let mut loaded_keys = props.loaded_keys;
    let grid_layout = props.grid_layout;
    let selected_from_research = props.selected_from_research;
    let selected_from_uprooted = props.selected_from_uprooted;
    let tier_overrides = props.tier_overrides;
    let dragging_slot = props.dragging_slot;
    let drop_target_cell = props.drop_target_cell;
    let drag_follower = props.drag_follower;
    let active_container_slots = props.active_container_slots;
    let _ = selected_from_uprooted;
    let mut editing_target = use_signal::<Option<OverrideEditTarget>>(|| None);
    // True while the player has the alt-state mini grid open. Distinct
    // signal from `editing_target` because the position picker is a modal
    // overlay rather than a hotkey picker, but only one of the two should
    // be active at a time.
    let mut alt_position_picker_open = use_signal::<bool>(|| false);
    let mut upgrade_position_picker_open = use_signal::<bool>(|| false);
    let layout_snapshot = *grid_layout.read();
    let object_id_for_capture = detail.object_id();
    let is_command_for_capture = detail.is_command();
    let is_off_state_for_capture = detail.is_off_state();
    let upgrade_unit_id_for_capture = detail.upgrade_unit_id();
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

    let alt_hotkey_token_display = detail.alt_hotkey_token();
    let alt_hotkey_display = alt_hotkey_token_display
        .map(|token| token.display_label())
        .unwrap_or_default();
    let alt_hotkey_is_editing = editing_snapshot == Some(OverrideEditTarget::AltHotkey);
    let alt_hotkey_label = if alt_hotkey_display.is_empty() {
        String::from("\u{2013}")
    } else {
        alt_hotkey_display.clone()
    };
    let alt_hotkey_is_special_token = alt_hotkey_token_display
        .map(|token| char::try_from(token).is_err())
        .unwrap_or(false);

    let total_tier_count: usize = detail
        .ubertip_levels()
        .len()
        .max(detail.name_levels().len())
        .max(detail.icon_levels_len());
    let stored_tier_index = tier_overrides
        .read()
        .get(detail.object_id().value())
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
    let object_id_text = detail.object_id().value();

    // Auto-scroll the override card into view when its detail changes (i.e. when
    // a different tile gets selected). On phone / tablet widths the override
    // sits below the command grids, and tapping a tile near the bottom of the
    // viewport otherwise leaves the override below the fold. The viewport-width
    // gate keeps desktop selections from yanking the page mid-interaction;
    // `documentElement.clientWidth` is preferred over `Window::inner_width` so
    // the threshold matches the CSS breakpoint (which excludes scrollbars).
    let scroll_dependency = detail.object_id();
    use_effect(move || {
        let _track = scroll_dependency;
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
    let picker_effective_object_id: WarcraftObjectId =
        if matches!(picker_target, Some(OverrideEditTarget::UpgradeHotkey)) {
            upgrade_unit_id_for_capture.unwrap_or(object_id_for_capture)
        } else {
            object_id_for_capture
        };
    let picker_rows: Vec<Vec<KeyPickerCell>> = if picker_open {
        PickerRows::build(
            layout_snapshot,
            &active_container_slots,
            picker_effective_object_id.value(),
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
    let picker_object_id = picker_effective_object_id;

    let handle_hotkey_activate = move |_| editing_target.set(Some(OverrideEditTarget::Hotkey));
    let handle_research_activate =
        move |_| editing_target.set(Some(OverrideEditTarget::ResearchHotkey));
    let handle_alt_position_click = move |_| alt_position_picker_open.set(true);
    let handle_alt_activate = move |_| editing_target.set(Some(OverrideEditTarget::AltHotkey));
    let handle_upgrade_position_click = move |_| upgrade_position_picker_open.set(true);
    let handle_upgrade_activate =
        move |_| editing_target.set(Some(OverrideEditTarget::UpgradeHotkey));
    let handle_picker_close = move |_| editing_target.set(None);

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
            picker_object_id.value(),
            token,
            custom_keys_ref,
            layout_snapshot_for_check,
            is_research_check,
        );
        drop(read_guard);
        if conflict.is_some() {
            return;
        }
        let hotkey_target = match active_target {
            OverrideEditTarget::Hotkey if is_off_state_for_capture => {
                HotkeyTarget::ability_off_state(picker_object_id)
            }
            OverrideEditTarget::Hotkey if is_command_for_capture => {
                HotkeyTarget::command(picker_object_id)
            }
            OverrideEditTarget::Hotkey => HotkeyTarget::ability(picker_object_id),
            OverrideEditTarget::ResearchHotkey => HotkeyTarget::ability_research(picker_object_id),
            OverrideEditTarget::AltHotkey => HotkeyTarget::ability_off_state(picker_object_id),
            OverrideEditTarget::UpgradeHotkey => HotkeyTarget::ability(picker_object_id),
        };
        HotkeyOverride::apply(&mut loaded_keys, hotkey_target, Some(token));
        editing_target.set(None);
    };

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

    let upgrade_id_option = detail.upgrade_unit_id();
    let upgrade_hotkey_token = detail.upgrade_hotkey_token();
    let upgrade_hotkey_display = upgrade_hotkey_token
        .map(|token| token.display_label())
        .unwrap_or_default();
    let upgrade_is_editing = editing_snapshot == Some(OverrideEditTarget::UpgradeHotkey);
    let upgrade_hotkey_label = if upgrade_hotkey_display.is_empty() {
        String::from("\u{2013}")
    } else {
        upgrade_hotkey_display.clone()
    };
    let upgrade_hotkey_is_special = upgrade_hotkey_token
        .map(|token| char::try_from(token).is_err())
        .unwrap_or(false);

    let alt_picker_visible = *alt_position_picker_open.read();
    let alt_picker_object_id = object_id_for_capture;
    let alt_display_name = detail
        .alt_display_name()
        .map(str::to_owned)
        .unwrap_or_else(|| detail.display_name().to_string());
    let alt_picker_slots: Rc<[GridSlotId]> = if alt_picker_visible {
        let mut combined: Vec<GridSlotId> = Vec::with_capacity(active_container_slots.len() + 1);
        combined.push(GridSlotId::ability_off(alt_picker_object_id));
        for slot in active_container_slots.iter() {
            if let GridSlotId::Ability(ability_id) = slot
                && ability_id.object_id() == alt_picker_object_id
            {
                continue;
            }
            combined.push(*slot);
        }
        combined.into()
    } else {
        Rc::from([] as [GridSlotId; 0])
    };

    let upgrade_picker_visible = *upgrade_position_picker_open.read();
    let upgrade_display_name = detail
        .upgrade_display_name()
        .map(str::to_owned)
        .unwrap_or_else(|| String::from("Upgraded form"));
    let upgrade_picker_slots: Rc<[GridSlotId]> = if upgrade_picker_visible {
        if let Some(upgrade_id) = upgrade_unit_id_for_capture {
            let base_unit_id_for_filter = object_id_for_capture;
            let mut combined: Vec<GridSlotId> =
                Vec::with_capacity(active_container_slots.len() + 1);
            combined.push(GridSlotId::ability(upgrade_id));
            for slot in active_container_slots.iter() {
                if let GridSlotId::Ability(base_id) = slot
                    && base_id.object_id() == base_unit_id_for_filter
                {
                    continue;
                }
                combined.push(*slot);
            }
            combined.into()
        } else {
            Rc::from([] as [GridSlotId; 0])
        }
    } else {
        Rc::from([] as [GridSlotId; 0])
    };

    let is_info_only = detail.info_only();
    let hotkey_title = String::from("Hotkey");
    let research_hotkey_title = String::from("Research hotkey");

    rsx! {
        div { class: "tile-override-card",
            div { class: "tile-override-header",
                div { class: "tile-override-header-text",
                    h3 { class: "tile-override-name", {active_tier_name} }
                    code { class: "tile-override-id", {object_id_text} }
                }
                if show_hotkey_field {
                    OverrideKeyField {
                        label: hotkey_label,
                        is_editing: hotkey_is_editing,
                        is_special: hotkey_is_special_token,
                        title: hotkey_title,
                        on_activate: handle_hotkey_activate,
                    }
                } else if show_research_field {
                    OverrideKeyField {
                        label: research_label,
                        is_editing: research_is_editing,
                        is_special: research_is_special_token,
                        title: research_hotkey_title,
                        on_activate: handle_research_activate,
                    }
                } else if is_info_only {
                    p { class: "tile-override-info-only", "Passive racial ability" }
                }
            }
            if !primary_description_lines.is_empty() {
                AbilityDescription { description_lines: primary_description_lines }
            }
            if has_alt_state {
                AltStateSection {
                    alt_name_text,
                    alt_description_lines,
                    show_alt_controls,
                    alt_hotkey_label,
                    alt_hotkey_is_editing,
                    alt_hotkey_is_special_token,
                    on_position_click: handle_alt_position_click,
                    on_hotkey_activate: handle_alt_activate,
                }
            }
            if upgrade_id_option.is_some() && !is_research_context {
                UpgradeSection {
                    upgrade_hotkey_label,
                    upgrade_is_editing,
                    upgrade_hotkey_is_special,
                    is_research_context,
                    on_position_click: handle_upgrade_position_click,
                    on_hotkey_activate: handle_upgrade_activate,
                }
            }
            if has_multiple_tiers {
                UpgradeTierSelector {
                    object_id: detail.object_id(),
                    active_tier_index,
                    total_tier_count,
                    tier_label_text,
                    tier_overrides,
                }
            }
        }
        if picker_open {
            KeyPicker {
                title: picker_title,
                rows: picker_rows,
                open: true,
                on_pick,
                on_close: handle_picker_close,
            }
        }
        if alt_picker_visible {
            AltPositionPicker {
                object_id: alt_picker_object_id,
                display_name: alt_display_name,
                picker_slots: alt_picker_slots,
                loaded_keys,
                grid_layout,
                dragging_slot,
                drop_target_cell,
                drag_follower,
                alt_position_picker_open,
            }
        }
        if let Some(upgrade_id) = upgrade_unit_id_for_capture {
            if upgrade_picker_visible {
                UpgradePositionPicker {
                    upgrade_unit_id: upgrade_id,
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

struct PickerRows;

impl PickerRows {
    fn build(
        layout: GridLayout,
        container_slots: &[GridSlotId],
        target_object_id: &str,
        current_token: Option<HotkeyToken>,
        is_research_context: bool,
        custom_keys: Option<&CustomKeys>,
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
                            let display_name = conflict.display_name().to_string();
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
}
