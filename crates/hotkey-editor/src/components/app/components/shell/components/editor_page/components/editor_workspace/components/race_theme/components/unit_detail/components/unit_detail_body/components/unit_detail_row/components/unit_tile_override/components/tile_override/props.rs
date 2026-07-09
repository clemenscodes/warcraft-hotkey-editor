use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::HotkeyToken;
use warcraft_keybinds::InspectorDetail;

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
use super::logic::{AltContent, FieldVisibility, HotkeyFieldView, PickerTarget, TierResolution};
use super::state::OverrideEditTarget;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPickerCell;

/// The per-tile override editor: the panel that edits the selected ability's hotkey,
/// off-state, upgraded form, and command-card position. The shared editor signals it
/// drives are sourced from context by the component's hook; only the inspected detail
/// and its container slots are props.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideProps {
    pub detail: InspectorDetail,
    pub active_container_slots: Rc<[GridSlotId]>,
}

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

/// Every computed intermediate the override panel's props tree is built from. The
/// hook wires the signals and derivations into one of these; the whole nested props
/// tree then derives itself through the `From` impl below, so the hook never
/// assembles a props struct by hand.
pub(super) struct TileOverrideInputs {
    pub(super) object_id: WarcraftObjectId,
    pub(super) upgrade_unit_id: Option<WarcraftObjectId>,
    pub(super) visibility: FieldVisibility,
    pub(super) tiers: TierResolution,
    pub(super) alt_content: AltContent,
    pub(super) hotkey_view: HotkeyFieldView,
    pub(super) research_view: HotkeyFieldView,
    pub(super) alt_view: HotkeyFieldView,
    pub(super) upgrade_view: HotkeyFieldView,
    pub(super) editing: OverrideEditing,
    pub(super) pickers: PositionPickers,
    pub(super) picker_rows: Vec<Vec<KeyPickerCell>>,
    pub(super) alt_display_name: String,
    pub(super) upgrade_display_name: String,
    pub(super) alt_picker_slots: Rc<[GridSlotId]>,
    pub(super) upgrade_picker_slots: Rc<[GridSlotId]>,
    pub(super) tier_overrides: Signal<HashMap<WarcraftObjectId, usize>>,
}

impl From<TileOverrideInputs> for TileOverrideModel {
    fn from(inputs: TileOverrideInputs) -> Self {
        let TileOverrideInputs {
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
            tier_overrides,
        } = inputs;
        let name = TileOverrideNameProps {
            text: tiers.active_tier_name,
        };
        let id = TileOverrideIdProps { object_id };
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
        let key_picker = TileOverrideKeyPickerProps {
            visible: editing.picker.open,
            title: editing.picker.title,
            rows: picker_rows,
            on_pick: editing.on_pick,
            on_close: editing.on_close,
        };
        let alt_picker = TileOverrideAltPickerProps {
            object_id,
            display_name: alt_display_name,
            picker_slots: alt_picker_slots,
            alt_position_picker_open: pickers.alt_open,
        };
        let upgrade_picker = TileOverrideUpgradePickerProps {
            upgrade_unit_id,
            display_name: upgrade_display_name,
            picker_slots: upgrade_picker_slots,
            upgrade_position_picker_open: pickers.upgrade_open,
        };
        Self {
            card,
            key_picker,
            alt_picker,
            upgrade_picker,
        }
    }
}
