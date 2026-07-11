use super::view::TileOverrideView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::HotkeyToken;
use warcraft_keybinds::InspectorDetail;

use super::presentation::{AltContent, FieldVisibility, HotkeyFieldView, PickerTarget, TierResolution};
use super::state::OverrideEditTarget;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::KeyPickerCell;

/// The per-tile override editor: the panel that edits the selected ability's hotkey,
/// off-state, upgraded form, and command-card position. The shared editor signals it
/// drives are sourced from context by the component's hook; only the inspected detail
/// and its container slots are props.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideModel {
    pub detail: InspectorDetail,
    pub active_container_slots: Rc<[GridSlotId]>,
}

/// Everything the override panel body places, already shaped as domain values: the
/// card's fields flattened, plus each picker dialog's own fields. The body names each
/// child component and sets these fields directly — no child props are assembled here.
pub(super) struct TileOverridePresentation {
    pub(super) name_text: String,
    pub(super) object_id: WarcraftObjectId,
    pub(super) show_hotkey_field: bool,
    pub(super) hotkey_label: String,
    pub(super) hotkey_is_editing: bool,
    pub(super) hotkey_is_special: bool,
    pub(super) on_hotkey_activate: EventHandler<()>,
    pub(super) show_research_field: bool,
    pub(super) research_label: String,
    pub(super) research_is_editing: bool,
    pub(super) research_is_special: bool,
    pub(super) on_research_activate: EventHandler<()>,
    pub(super) is_info_only: bool,
    pub(super) description_lines: Vec<String>,
    pub(super) alt_name_text: Option<String>,
    pub(super) alt_description_lines: Vec<String>,
    pub(super) show_alt_controls: bool,
    pub(super) alt_hotkey_label: String,
    pub(super) alt_hotkey_is_editing: bool,
    pub(super) alt_hotkey_is_special_token: bool,
    pub(super) on_alt_position_click: EventHandler<()>,
    pub(super) on_alt_hotkey_activate: EventHandler<()>,
    pub(super) upgrade_show: bool,
    pub(super) upgrade_hotkey_label: String,
    pub(super) upgrade_is_editing: bool,
    pub(super) upgrade_hotkey_is_special: bool,
    pub(super) on_upgrade_position_click: EventHandler<()>,
    pub(super) on_upgrade_hotkey_activate: EventHandler<()>,
    pub(super) active_tier_index: usize,
    pub(super) total_tier_count: usize,
    pub(super) tier_label_text: String,
    pub(super) key_picker_visible: bool,
    pub(super) picker_title: String,
    pub(super) picker_rows: Vec<Vec<KeyPickerCell>>,
    pub(super) on_pick: EventHandler<HotkeyToken>,
    pub(super) on_close: EventHandler<()>,
    pub(super) alt_display_name: String,
    pub(super) alt_picker_slots: Rc<[GridSlotId]>,
    pub(super) alt_open: Signal<bool>,
    pub(super) upgrade_unit_id: Option<WarcraftObjectId>,
    pub(super) upgrade_display_name: String,
    pub(super) upgrade_picker_slots: Rc<[GridSlotId]>,
    pub(super) upgrade_open: Signal<bool>,
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
}

impl From<TileOverrideInputs> for TileOverridePresentation {
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
        } = inputs;
        Self {
            name_text: tiers.active_tier_name,
            object_id,
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
            description_lines: tiers.description_lines,
            alt_name_text: alt_content.name_text,
            alt_description_lines: alt_content.description_lines,
            show_alt_controls: visibility.show_alt_controls,
            alt_hotkey_label: alt_view.label,
            alt_hotkey_is_editing: alt_view.is_editing,
            alt_hotkey_is_special_token: alt_view.is_special,
            on_alt_position_click: pickers.on_alt_position_click,
            on_alt_hotkey_activate: editing.on_alt_activate,
            upgrade_show: visibility.upgrade_show,
            upgrade_hotkey_label: upgrade_view.label,
            upgrade_is_editing: upgrade_view.is_editing,
            upgrade_hotkey_is_special: upgrade_view.is_special,
            on_upgrade_position_click: pickers.on_upgrade_position_click,
            on_upgrade_hotkey_activate: editing.on_upgrade_activate,
            active_tier_index: tiers.active_tier_index,
            total_tier_count: tiers.total_tier_count,
            tier_label_text: tiers.tier_label_text,
            key_picker_visible: editing.picker.open,
            picker_title: editing.picker.title,
            picker_rows,
            on_pick: editing.on_pick,
            on_close: editing.on_close,
            alt_display_name,
            alt_picker_slots,
            alt_open: pickers.alt_open,
            upgrade_unit_id,
            upgrade_display_name,
            upgrade_picker_slots,
            upgrade_open: pickers.upgrade_open,
        }
    }
}

impl From<&TileOverrideView> for TileOverrideModel {
    fn from(view: &TileOverrideView) -> Self {
        let TileOverrideView {
            detail,
            active_container_slots,
        } = view.clone();
        Self {
            detail,
            active_container_slots,
        }
    }
}

impl ddd::Model for TileOverrideModel {
    type View = TileOverrideView;
}
