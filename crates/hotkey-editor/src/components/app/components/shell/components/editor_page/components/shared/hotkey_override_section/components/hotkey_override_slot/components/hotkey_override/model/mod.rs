use super::view::HotkeyOverrideView;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::HotkeyToken;
use warcraft_keybinds::InspectorDetail;

use super::presentation::{AltContent, FieldVisibility, HotkeyFieldView, PickerTarget, TierResolution};
use super::state::OverrideEditTarget;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::KeyPickerCell;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyOverrideModel {
    pub detail: InspectorDetail,
    pub active_container_slots: Rc<[GridSlotId]>,
}

pub(super) struct HotkeyOverridePresentation {
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
    pub(super) alt_name_text: Option<String>,
    pub(super) show_alt_controls: bool,
    pub(super) alt_hotkey_label: String,
    pub(super) alt_hotkey_is_editing: bool,
    pub(super) alt_hotkey_is_special_token: bool,
    pub(super) on_hotkey_alt_position_click: EventHandler<()>,
    pub(super) on_alt_hotkey_activate: EventHandler<()>,
    pub(super) upgrade_show: bool,
    pub(super) upgrade_hotkey_label: String,
    pub(super) upgrade_is_editing: bool,
    pub(super) upgrade_hotkey_is_special: bool,
    pub(super) on_hotkey_upgrade_position_click: EventHandler<()>,
    pub(super) on_upgrade_hotkey_activate: EventHandler<()>,
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

pub(super) struct PositionPickers {
    pub(super) alt_open: Signal<bool>,
    pub(super) upgrade_open: Signal<bool>,
    pub(super) on_hotkey_alt_position_click: EventHandler<()>,
    pub(super) on_hotkey_upgrade_position_click: EventHandler<()>,
}

pub(super) struct HotkeyOverrideInputs {
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

impl From<HotkeyOverrideInputs> for HotkeyOverridePresentation {
    fn from(inputs: HotkeyOverrideInputs) -> Self {
        let HotkeyOverrideInputs {
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
            alt_name_text: alt_content.name_text,
            show_alt_controls: visibility.show_alt_controls,
            alt_hotkey_label: alt_view.label,
            alt_hotkey_is_editing: alt_view.is_editing,
            alt_hotkey_is_special_token: alt_view.is_special,
            on_hotkey_alt_position_click: pickers.on_hotkey_alt_position_click,
            on_alt_hotkey_activate: editing.on_alt_activate,
            upgrade_show: visibility.upgrade_show,
            upgrade_hotkey_label: upgrade_view.label,
            upgrade_is_editing: upgrade_view.is_editing,
            upgrade_hotkey_is_special: upgrade_view.is_special,
            on_hotkey_upgrade_position_click: pickers.on_hotkey_upgrade_position_click,
            on_upgrade_hotkey_activate: editing.on_upgrade_activate,
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

impl From<&HotkeyOverrideView> for HotkeyOverrideModel {
    fn from(view: &HotkeyOverrideView) -> Self {
        let HotkeyOverrideView {
            detail,
            active_container_slots,
        } = view.clone();
        Self {
            detail,
            active_container_slots,
        }
    }
}

impl ddd::Model for HotkeyOverrideModel {
    type View = HotkeyOverrideView;
}
