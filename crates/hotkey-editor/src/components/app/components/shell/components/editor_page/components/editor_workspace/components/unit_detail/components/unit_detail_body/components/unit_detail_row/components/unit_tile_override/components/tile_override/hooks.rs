use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::text::description::Description;
use warcraft_keybinds::text::tip::Tip;
use warcraft_keybinds::{CustomKeys, GridLayout, GridSlotId, HotkeyTarget, HotkeyToken, Letter};
use wasm_bindgen::JsCast;

use super::components::tile_override_alt_picker::TileOverrideAltPickerProps;
use super::components::tile_override_card::components::ability_description::AbilityDescriptionProps;
use super::components::tile_override_card::components::alt_state_section::AltStateSectionProps;
use super::components::tile_override_card::components::tile_override_header::components::tile_override_header_text::components::tile_override_id::TileOverrideIdProps;
use super::components::tile_override_card::components::tile_override_header::components::tile_override_header_text::components::tile_override_name::TileOverrideNameProps;
use super::components::tile_override_card::components::tile_override_header::components::tile_override_header_text::TileOverrideHeaderTextProps;
use super::components::tile_override_card::components::tile_override_header::components::tile_override_hotkey_slot::TileOverrideHotkeySlotProps;
use super::components::tile_override_card::components::tile_override_header::TileOverrideHeaderProps;
use super::components::tile_override_card::components::upgrade_section::UpgradeSectionProps;
use super::components::tile_override_card::components::upgrade_tier::UpgradeTierProps;
use super::components::tile_override_card::TileOverrideCardProps;
use super::components::tile_override_key_picker::TileOverrideKeyPickerProps;
use super::components::tile_override_upgrade_picker::TileOverrideUpgradePickerProps;
use super::props::TileOverrideProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::{KeyPickerCell, KeyPickerCellState};
use crate::services::customkeys::hotkey_override::HotkeyOverride;
use crate::services::customkeys::context::use_custom_keys_service;

/// Which field the hotkey picker is currently editing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OverrideEditTarget {
    Hotkey,
    ResearchHotkey,
    /// Off-state hotkey of a toggle ability — Stop Defend, Unburrow, unmorph. Writes
    /// the `Unhotkey` field rather than `Hotkey`.
    AltHotkey,
    /// Hotkey for the upgraded-form unit that shares this button position (e.g.
    /// post-Barrage Siege Engine). Writes to the upgrade unit's own `Hotkey=` binding.
    UpgradeHotkey,
}

/// Everything the override panel body places, already shaped: the card's full
/// nested props tree plus the three picker dialogs.
pub(super) struct TileOverrideModel {
    pub(super) card: TileOverrideCardProps,
    pub(super) key_picker: TileOverrideKeyPickerProps,
    pub(super) alt_picker: TileOverrideAltPickerProps,
    pub(super) upgrade_picker: TileOverrideUpgradePickerProps,
}

fn label_or_dash(display: &str) -> String {
    if display.is_empty() {
        String::from("\u{2013}")
    } else {
        display.to_string()
    }
}

pub(super) fn use_tile_override(props: &TileOverrideProps) -> TileOverrideModel {
    let detail = props.detail.clone();
    let loaded_keys = props.loaded_keys;
    let custom_keys_service = use_custom_keys_service();
    let grid_layout = props.grid_layout;
    let selected_from_research = props.selected_from_research;
    let tier_overrides = props.tier_overrides;
    let active_container_slots = props.active_container_slots.clone();
    let mut editing_target = use_signal::<Option<OverrideEditTarget>>(|| None);
    let alt_position_picker_open = use_signal::<bool>(|| false);
    let upgrade_position_picker_open = use_signal::<bool>(|| false);
    let layout_snapshot = *grid_layout.read();
    let object_id_for_capture = detail.object_id();
    let is_command_for_capture = detail.is_command();
    let is_off_state_for_capture = detail.is_off_state();
    let upgrade_unit_id_for_capture = detail.upgrade_unit_id();
    let layout_derived_hotkey_token = detail
        .button_position()
        .and_then(|position| layout_snapshot.letter_at(position.column(), position.row()))
        .and_then(|character| HotkeyToken::try_from(character).ok());
    let layout_derived_research_token = detail
        .research_button_position()
        .or(detail.button_position())
        .and_then(|position| layout_snapshot.letter_at(position.column(), position.row()))
        .and_then(|character| HotkeyToken::try_from(character).ok());
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
    let show_hotkey_field = !detail.is_passive() && (!is_research_context || detail.is_command());
    let show_research_field = !detail.is_command() && is_research_context && !detail.info_only();
    let mut hotkey_assign_request = props.hotkey_assign_request;
    let hotkey_field_available = show_hotkey_field;
    let research_field_available = show_research_field;
    use_effect(move || {
        if !*hotkey_assign_request.read() {
            return;
        }
        if hotkey_field_available {
            editing_target.set(Some(OverrideEditTarget::Hotkey));
        } else if research_field_available {
            editing_target.set(Some(OverrideEditTarget::ResearchHotkey));
        }
        hotkey_assign_request.set(false);
    });
    let editing_snapshot = *editing_target.read();
    let hotkey_is_editing = editing_snapshot == Some(OverrideEditTarget::Hotkey);
    let research_is_editing = editing_snapshot == Some(OverrideEditTarget::ResearchHotkey);
    let hotkey_label = label_or_dash(&hotkey_display);
    let research_label = label_or_dash(&research_hotkey_display);
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
    let alt_hotkey_label = label_or_dash(&alt_hotkey_display);
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
    let object_id_text = detail.object_id().value().to_string();
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
    let on_hotkey_activate = EventHandler::new(move |_: ()| {
        editing_target.set(Some(OverrideEditTarget::Hotkey));
    });
    let on_research_activate = EventHandler::new(move |_: ()| {
        editing_target.set(Some(OverrideEditTarget::ResearchHotkey));
    });
    let mut alt_open_for_click = alt_position_picker_open;
    let on_alt_position_click = EventHandler::new(move |_: ()| alt_open_for_click.set(true));
    let on_alt_activate = EventHandler::new(move |_: ()| {
        editing_target.set(Some(OverrideEditTarget::AltHotkey));
    });
    let mut upgrade_open_for_click = upgrade_position_picker_open;
    let on_upgrade_position_click =
        EventHandler::new(move |_: ()| upgrade_open_for_click.set(true));
    let on_upgrade_activate = EventHandler::new(move |_: ()| {
        editing_target.set(Some(OverrideEditTarget::UpgradeHotkey));
    });
    let on_picker_close = EventHandler::new(move |_: ()| editing_target.set(None));
    let on_pick = EventHandler::new(move |token: HotkeyToken| {
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
        custom_keys_service.override_hotkey(hotkey_target, Some(token));
        editing_target.set(None);
    });
    let alt_name_text = detail.alt_display_name().map(str::to_owned);
    let alt_description_lines: Vec<String> = detail
        .alt_ubertip()
        .map(Description::lines_from)
        .unwrap_or_default();
    let has_alt_state = alt_name_text.is_some() || !alt_description_lines.is_empty();
    let show_alt_controls = has_alt_state && !is_research_context && !detail.is_command();
    let upgrade_id_option = detail.upgrade_unit_id();
    let upgrade_show = upgrade_id_option.is_some() && !is_research_context;
    let upgrade_hotkey_token = detail.upgrade_hotkey_token();
    let upgrade_hotkey_display = upgrade_hotkey_token
        .map(|token| token.display_label())
        .unwrap_or_default();
    let upgrade_is_editing = editing_snapshot == Some(OverrideEditTarget::UpgradeHotkey);
    let upgrade_hotkey_label = label_or_dash(&upgrade_hotkey_display);
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

    let name = TileOverrideNameProps {
        text: active_tier_name,
    };
    let id = TileOverrideIdProps {
        text: object_id_text,
    };
    let hotkey_slot = TileOverrideHotkeySlotProps {
        show_hotkey_field,
        hotkey_label,
        hotkey_is_editing,
        hotkey_is_special: hotkey_is_special_token,
        on_hotkey_activate,
        show_research_field,
        research_label,
        research_is_editing,
        research_is_special: research_is_special_token,
        on_research_activate,
        is_info_only,
    };
    let description = AbilityDescriptionProps {
        description_lines: primary_description_lines,
    };
    let alt_state = AltStateSectionProps {
        alt_name_text,
        alt_description_lines,
        show_alt_controls,
        alt_hotkey_label,
        alt_hotkey_is_editing,
        alt_hotkey_is_special_token,
        on_position_click: on_alt_position_click,
        on_hotkey_activate: on_alt_activate,
    };
    let upgrade = UpgradeSectionProps {
        show: upgrade_show,
        upgrade_hotkey_label,
        upgrade_is_editing,
        upgrade_hotkey_is_special,
        on_position_click: on_upgrade_position_click,
        on_hotkey_activate: on_upgrade_activate,
    };
    let tier = UpgradeTierProps {
        object_id: detail.object_id(),
        active_tier_index,
        total_tier_count,
        tier_label_text,
        tier_overrides,
    };
    let key_picker = TileOverrideKeyPickerProps {
        visible: picker_open,
        title: picker_title,
        rows: picker_rows,
        on_pick,
        on_close: on_picker_close,
    };
    let alt_picker = TileOverrideAltPickerProps {
        visible: alt_picker_visible,
        object_id: alt_picker_object_id,
        display_name: alt_display_name,
        picker_slots: alt_picker_slots,
        loaded_keys: props.loaded_keys,
        grid_layout: props.grid_layout,
        dragging_slot: props.dragging_slot,
        drop_target_tile: props.drop_target_tile,
        drag_follower: props.drag_follower,
        alt_position_picker_open,
    };
    let upgrade_picker = TileOverrideUpgradePickerProps {
        visible: upgrade_picker_visible,
        upgrade_unit_id: upgrade_unit_id_for_capture,
        display_name: upgrade_display_name,
        picker_slots: upgrade_picker_slots,
        loaded_keys: props.loaded_keys,
        grid_layout: props.grid_layout,
        dragging_slot: props.dragging_slot,
        drop_target_tile: props.drop_target_tile,
        drag_follower: props.drag_follower,
        upgrade_position_picker_open,
    };
    let header_text = TileOverrideHeaderTextProps { name, id };
    let header = TileOverrideHeaderProps {
        header_text,
        hotkey_slot,
    };
    let card = TileOverrideCardProps {
        header,
        description,
        alt_state,
        upgrade,
        tier,
    };
    TileOverrideModel {
        card,
        key_picker,
        alt_picker,
        upgrade_picker,
    }
}

const PICKER_ROWS: &[&[HotkeyToken]] = &[
    &[
        HotkeyToken::Letter(Letter::Q),
        HotkeyToken::Letter(Letter::W),
        HotkeyToken::Letter(Letter::E),
        HotkeyToken::Letter(Letter::R),
        HotkeyToken::Letter(Letter::T),
        HotkeyToken::Letter(Letter::Y),
        HotkeyToken::Letter(Letter::U),
        HotkeyToken::Letter(Letter::I),
        HotkeyToken::Letter(Letter::O),
        HotkeyToken::Letter(Letter::P),
    ],
    &[
        HotkeyToken::Letter(Letter::A),
        HotkeyToken::Letter(Letter::S),
        HotkeyToken::Letter(Letter::D),
        HotkeyToken::Letter(Letter::F),
        HotkeyToken::Letter(Letter::G),
        HotkeyToken::Letter(Letter::H),
        HotkeyToken::Letter(Letter::J),
        HotkeyToken::Letter(Letter::K),
        HotkeyToken::Letter(Letter::L),
    ],
    &[
        HotkeyToken::Letter(Letter::Z),
        HotkeyToken::Letter(Letter::X),
        HotkeyToken::Letter(Letter::C),
        HotkeyToken::Letter(Letter::V),
        HotkeyToken::Letter(Letter::B),
        HotkeyToken::Letter(Letter::N),
        HotkeyToken::Letter(Letter::M),
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
