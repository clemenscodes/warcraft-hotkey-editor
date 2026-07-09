use dioxus::prelude::*;
use warcraft_keybinds::{HotkeyTarget, HotkeyToken};

use super::logic::{
    AltContent, FieldVisibility, FieldVisibilityInputs, HotkeyFieldView, OverridePickerSlots,
    OverrideTokens, OverrideTokensInputs, PickerBoard, PickerContext, PickerTarget,
    PickerTargetInputs, TierResolution, TierResolutionInputs,
};
use super::props::{
    OverrideEditing, PositionPickers, TileOverrideInputs, TileOverrideModel, TileOverrideProps,
};
use super::state::OverrideEditTarget;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::customkeys::hotkey_override::HotkeyOverride;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;

fn use_override_editing(
    props: &TileOverrideProps,
    visibility: &FieldVisibility,
    tokens: &OverrideTokens,
) -> OverrideEditing {
    let detail = props.detail.clone();
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let editor = use_editor_state();
    let custom_keys_service = use_custom_keys_service();
    let active_container_slots = props.active_container_slots.clone();
    let mut editing_target = use_signal::<Option<OverrideEditTarget>>(|| None);
    let mut hotkey_assign_request = editor.hotkey_assign_request();
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
    let picker_inputs = PickerTargetInputs {
        snapshot,
        tokens,
        object_id,
        upgrade_unit_id,
    };
    let picker = PickerTarget::from(picker_inputs);

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
        let conflict = HotkeyOverride::detect_conflict(
            &picker_active_container,
            picker_object_id,
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
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let editor = use_editor_state();
    let tier_overrides = editor.tier_overrides();
    let is_research_context = *editor.selected_from_research().read();
    let layout_snapshot = *grid_layout.read();
    let object_id = detail.object_id();
    let upgrade_unit_id = detail.upgrade_unit_id();

    let tokens_inputs = OverrideTokensInputs {
        detail: &detail,
        layout: layout_snapshot,
    };
    let tokens = OverrideTokens::from(tokens_inputs);
    let alt_content = AltContent::from(&detail);
    let visibility_inputs = FieldVisibilityInputs {
        detail: &detail,
        is_research_context,
        alt_content: &alt_content,
    };
    let visibility = FieldVisibility::from(visibility_inputs);
    let stored_tier_index = tier_overrides.read().get(&object_id).copied().unwrap_or(0);
    let tier_inputs = TierResolutionInputs {
        detail: &detail,
        stored_tier_index,
        is_research_context,
    };
    let tiers = TierResolution::from(tier_inputs);

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

    let inputs = TileOverrideInputs {
        object_id,
        upgrade_unit_id,
        visibility,
        tiers,
        alt_content,
        hotkey_view,
        research_view,
        alt_view,
        upgrade_view,
        editing,
        pickers,
        picker_rows,
        alt_display_name,
        upgrade_display_name,
        alt_picker_slots,
        upgrade_picker_slots,
    };
    TileOverrideModel::from(inputs)
}
