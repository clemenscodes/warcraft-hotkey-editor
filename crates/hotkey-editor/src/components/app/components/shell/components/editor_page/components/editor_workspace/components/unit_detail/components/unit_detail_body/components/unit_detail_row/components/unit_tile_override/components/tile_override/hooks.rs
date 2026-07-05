use dioxus::prelude::*;
use warcraft_keybinds::{HotkeyTarget, HotkeyToken};

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
use super::logic::{
    AltContent, FieldVisibility, HotkeyFieldView, OverridePickerSlots, OverrideTokens, PickerBoard,
    PickerContext, PickerTarget, TierResolution,
};
use super::props::TileOverrideProps;
use super::state::OverrideEditTarget;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::customkeys::hotkey_override::HotkeyOverride;

/// Everything the override panel body places, already shaped: the card's full
/// nested props tree plus the three picker dialogs.
pub(super) struct TileOverrideModel {
    pub(super) card: TileOverrideCardProps,
    pub(super) key_picker: TileOverrideKeyPickerProps,
    pub(super) alt_picker: TileOverrideAltPickerProps,
    pub(super) upgrade_picker: TileOverrideUpgradePickerProps,
}

/// The editing state of the override panel: which field the picker is open on, plus
/// every handler that opens, commits, or closes it. Owns the `editing_target` signal
/// and the assign-request effect; the commit handler runs the conflict check and
/// routes the write through the [`CustomKeysService`](crate::services::customkeys).
pub(super) struct OverrideEditing {
    pub(super) snapshot: Option<OverrideEditTarget>,
    pub(super) picker: PickerTarget,
    pub(super) on_hotkey_activate: EventHandler<()>,
    pub(super) on_research_activate: EventHandler<()>,
    pub(super) on_alt_activate: EventHandler<()>,
    pub(super) on_upgrade_activate: EventHandler<()>,
    pub(super) on_pick: EventHandler<HotkeyToken>,
    pub(super) on_close: EventHandler<()>,
}

/// The two off-form position pickers' open state and their open handlers.
pub(super) struct PositionPickers {
    pub(super) alt_open: Signal<bool>,
    pub(super) upgrade_open: Signal<bool>,
    pub(super) on_alt_position_click: EventHandler<()>,
    pub(super) on_upgrade_position_click: EventHandler<()>,
}

fn use_override_editing(
    props: &TileOverrideProps,
    visibility: &FieldVisibility,
    tokens: &OverrideTokens,
) -> OverrideEditing {
    let detail = props.detail.clone();
    let loaded_keys = props.loaded_keys;
    let grid_layout = props.grid_layout;
    let custom_keys_service = use_custom_keys_service();
    let active_container_slots = props.active_container_slots.clone();
    let mut editing_target = use_signal::<Option<OverrideEditTarget>>(|| None);
    let mut hotkey_assign_request = props.hotkey_assign_request;
    let hotkey_field_available = visibility.show_hotkey_field;
    let research_field_available = visibility.show_research_field;
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

    let snapshot = *editing_target.read();
    let object_id = detail.object_id();
    let upgrade_unit_id = detail.upgrade_unit_id();
    let is_off_state = detail.is_off_state();
    let is_command = detail.is_command();
    let picker = PickerTarget::resolve(snapshot, tokens, object_id, upgrade_unit_id);

    let on_hotkey_activate =
        EventHandler::new(move |_: ()| editing_target.set(Some(OverrideEditTarget::Hotkey)));
    let on_research_activate = EventHandler::new(move |_: ()| {
        editing_target.set(Some(OverrideEditTarget::ResearchHotkey))
    });
    let on_alt_activate =
        EventHandler::new(move |_: ()| editing_target.set(Some(OverrideEditTarget::AltHotkey)));
    let on_upgrade_activate =
        EventHandler::new(move |_: ()| editing_target.set(Some(OverrideEditTarget::UpgradeHotkey)));
    let on_close = EventHandler::new(move |_: ()| editing_target.set(None));

    let picker_active_container = active_container_slots.clone();
    let picker_object_id = picker.effective_object_id;
    let on_pick = EventHandler::new(move |token: HotkeyToken| {
        let Some(active_target) = *editing_target.read() else {
            return;
        };
        let layout_snapshot_for_check = *grid_layout.peek();
        let is_research_check = matches!(active_target, OverrideEditTarget::ResearchHotkey);
        let read_guard = loaded_keys.peek();
        let custom_keys_ref = read_guard.as_ref();
        let target_id = picker_object_id.value();
        let conflict = HotkeyOverride::detect_conflict(
            &picker_active_container,
            target_id,
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
            OverrideEditTarget::Hotkey if is_off_state => {
                HotkeyTarget::ability_off_state(picker_object_id)
            }
            OverrideEditTarget::Hotkey if is_command => HotkeyTarget::command(picker_object_id),
            OverrideEditTarget::Hotkey => HotkeyTarget::ability(picker_object_id),
            OverrideEditTarget::ResearchHotkey => HotkeyTarget::ability_research(picker_object_id),
            OverrideEditTarget::AltHotkey => HotkeyTarget::ability_off_state(picker_object_id),
            OverrideEditTarget::UpgradeHotkey => HotkeyTarget::ability(picker_object_id),
        };
        let selected_token = Some(token);
        custom_keys_service.override_hotkey(hotkey_target, selected_token);
        editing_target.set(None);
    });

    OverrideEditing {
        snapshot,
        picker,
        on_hotkey_activate,
        on_research_activate,
        on_alt_activate,
        on_upgrade_activate,
        on_pick,
        on_close,
    }
}

fn use_position_pickers() -> PositionPickers {
    let alt_open = use_signal::<bool>(|| false);
    let upgrade_open = use_signal::<bool>(|| false);
    let mut alt_open_for_click = alt_open;
    let on_alt_position_click = EventHandler::new(move |_: ()| alt_open_for_click.set(true));
    let mut upgrade_open_for_click = upgrade_open;
    let on_upgrade_position_click =
        EventHandler::new(move |_: ()| upgrade_open_for_click.set(true));
    PositionPickers {
        alt_open,
        upgrade_open,
        on_alt_position_click,
        on_upgrade_position_click,
    }
}

pub(super) fn use_tile_override(props: &TileOverrideProps) -> TileOverrideModel {
    let detail = props.detail.clone();
    let loaded_keys = props.loaded_keys;
    let grid_layout = props.grid_layout;
    let tier_overrides = props.tier_overrides;
    let is_research_context = *props.selected_from_research.read();
    let layout_snapshot = *grid_layout.read();
    let object_id = detail.object_id();
    let upgrade_unit_id = detail.upgrade_unit_id();

    let tokens = OverrideTokens::resolve(&detail, layout_snapshot);
    let alt_content = AltContent::from(&detail);
    let visibility = FieldVisibility::resolve(&detail, is_research_context, &alt_content);
    let stored_tier_index = tier_overrides
        .read()
        .get(object_id.value())
        .copied()
        .unwrap_or(0);
    let tiers = TierResolution::resolve(&detail, stored_tier_index, is_research_context);

    let editing = use_override_editing(props, &visibility, &tokens);
    let pickers = use_position_pickers();

    let hotkey_is_editing = editing.snapshot == Some(OverrideEditTarget::Hotkey);
    let research_is_editing = editing.snapshot == Some(OverrideEditTarget::ResearchHotkey);
    let alt_is_editing = editing.snapshot == Some(OverrideEditTarget::AltHotkey);
    let upgrade_is_editing = editing.snapshot == Some(OverrideEditTarget::UpgradeHotkey);
    let hotkey_view = HotkeyFieldView::new(tokens.hotkey, hotkey_is_editing);
    let research_view = HotkeyFieldView::new(tokens.research, research_is_editing);
    let alt_view = HotkeyFieldView::new(tokens.alt, alt_is_editing);
    let upgrade_view = HotkeyFieldView::new(tokens.upgrade, upgrade_is_editing);

    let picker_rows = if editing.picker.open {
        let container_slots = props.active_container_slots.clone();
        let picker_context = PickerContext {
            layout: layout_snapshot,
            container_slots,
            target_object_id: editing.picker.effective_object_id,
            current_token: editing.picker.current_token,
            is_research_context: editing.picker.is_research_context,
        };
        let read_guard = loaded_keys.read();
        let custom_keys = read_guard.as_ref();
        let board = PickerBoard::build(&picker_context, custom_keys);
        board.into_rows()
    } else {
        Vec::new()
    };

    let alt_picker_visible = *pickers.alt_open.read();
    let upgrade_picker_visible = *pickers.upgrade_open.read();
    let alt_display_name = detail
        .alt_display_name()
        .map(str::to_owned)
        .unwrap_or_else(|| detail.display_name().to_string());
    let upgrade_display_name = detail
        .upgrade_display_name()
        .map(str::to_owned)
        .unwrap_or_else(|| String::from("Upgraded form"));
    let alt_picker_slots = if alt_picker_visible {
        let built = OverridePickerSlots::alt(object_id, &props.active_container_slots);
        built.into_slots()
    } else {
        OverridePickerSlots::default().into_slots()
    };
    let upgrade_picker_slots = match (upgrade_picker_visible, upgrade_unit_id) {
        (true, Some(upgrade_id)) => {
            let built =
                OverridePickerSlots::upgrade(upgrade_id, object_id, &props.active_container_slots);
            built.into_slots()
        }
        _ => OverridePickerSlots::default().into_slots(),
    };

    let object_id_text = object_id.value().to_string();
    let name = TileOverrideNameProps {
        text: tiers.active_tier_name,
    };
    let id = TileOverrideIdProps {
        text: object_id_text,
    };
    let hotkey_slot = TileOverrideHotkeySlotProps {
        show_hotkey_field: visibility.show_hotkey_field,
        hotkey_label: hotkey_view.label,
        hotkey_is_editing: hotkey_view.is_editing,
        hotkey_is_special: hotkey_view.is_special,
        on_hotkey_activate: editing.on_hotkey_activate,
        show_research_field: visibility.show_research_field,
        research_label: research_view.label,
        research_is_editing: research_view.is_editing,
        research_is_special: research_view.is_special,
        on_research_activate: editing.on_research_activate,
        is_info_only: visibility.is_info_only,
    };
    let description = AbilityDescriptionProps {
        description_lines: tiers.description_lines,
    };
    let alt_state = AltStateSectionProps {
        alt_name_text: alt_content.name_text,
        alt_description_lines: alt_content.description_lines,
        show_alt_controls: visibility.show_alt_controls,
        alt_hotkey_label: alt_view.label,
        alt_hotkey_is_editing: alt_view.is_editing,
        alt_hotkey_is_special_token: alt_view.is_special,
        on_position_click: pickers.on_alt_position_click,
        on_hotkey_activate: editing.on_alt_activate,
    };
    let upgrade = UpgradeSectionProps {
        show: visibility.upgrade_show,
        upgrade_hotkey_label: upgrade_view.label,
        upgrade_is_editing: upgrade_view.is_editing,
        upgrade_hotkey_is_special: upgrade_view.is_special,
        on_position_click: pickers.on_upgrade_position_click,
        on_hotkey_activate: editing.on_upgrade_activate,
    };
    let tier = UpgradeTierProps {
        object_id,
        active_tier_index: tiers.active_tier_index,
        total_tier_count: tiers.total_tier_count,
        tier_label_text: tiers.tier_label_text,
        tier_overrides,
    };
    let key_picker = TileOverrideKeyPickerProps {
        visible: editing.picker.open,
        title: editing.picker.title,
        rows: picker_rows,
        on_pick: editing.on_pick,
        on_close: editing.on_close,
    };
    let alt_picker = TileOverrideAltPickerProps {
        visible: alt_picker_visible,
        object_id,
        display_name: alt_display_name,
        picker_slots: alt_picker_slots,
        loaded_keys: props.loaded_keys,
        grid_layout: props.grid_layout,
        dragging_slot: props.dragging_slot,
        drop_target_tile: props.drop_target_tile,
        drag_follower: props.drag_follower,
        alt_position_picker_open: pickers.alt_open,
    };
    let upgrade_picker = TileOverrideUpgradePickerProps {
        visible: upgrade_picker_visible,
        upgrade_unit_id,
        display_name: upgrade_display_name,
        picker_slots: upgrade_picker_slots,
        loaded_keys: props.loaded_keys,
        grid_layout: props.grid_layout,
        dragging_slot: props.dragging_slot,
        drop_target_tile: props.drop_target_tile,
        drag_follower: props.drag_follower,
        upgrade_position_picker_open: pickers.upgrade_open,
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
