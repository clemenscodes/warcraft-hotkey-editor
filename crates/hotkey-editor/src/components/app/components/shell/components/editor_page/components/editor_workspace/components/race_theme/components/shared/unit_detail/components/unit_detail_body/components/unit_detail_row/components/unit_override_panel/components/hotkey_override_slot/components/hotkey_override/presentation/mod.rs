use super::model::{
    OverrideEditing, PositionPickers, HotkeyOverrideInputs, HotkeyOverrideModel,
    HotkeyOverridePresentation,
};
use super::state::OverrideEditTarget;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::{KeyPickerCell, KeyPickerCellState};
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::customkeys::hotkey_override;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_api::Description;
use warcraft_api::Tip;
use warcraft_keybinds::{
    CustomKeys, GridLayout, GridSlotId, HotkeyTarget, HotkeyToken, InspectorDetail, Letter,
};

/// The four hotkey tokens this override can edit, resolved from the ability's own
/// bindings with a fallback to whatever letter the active grid layout assigns to the
/// ability's button position.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct OverrideTokens {
    pub(super) hotkey: Option<HotkeyToken>,
    pub(super) research: Option<HotkeyToken>,
    pub(super) alt: Option<HotkeyToken>,
    pub(super) upgrade: Option<HotkeyToken>,
}

/// The inputs that resolve the [`OverrideTokens`]: the inspected ability and the
/// active grid layout its fallback letters come from.
pub(super) struct OverrideTokensInputs<'a> {
    pub(super) detail: &'a InspectorDetail,
    pub(super) layout: GridLayout,
}

impl From<OverrideTokensInputs<'_>> for OverrideTokens {
    fn from(inputs: OverrideTokensInputs<'_>) -> Self {
        let OverrideTokensInputs { detail, layout } = inputs;
        let hotkey = detail.effective_hotkey(&layout);
        let research = detail.effective_research_hotkey(&layout);
        let alt = detail.alt_hotkey_token();
        let upgrade = detail.upgrade_hotkey_token();
        Self {
            hotkey,
            research,
            alt,
            upgrade,
        }
    }
}

/// The off-state ("alt") name and description text, shared by the visibility
/// calculation and the rendered alt-state section.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub(super) struct AltContent {
    pub(super) name_text: Option<String>,
    pub(super) description_lines: Vec<String>,
}

impl From<&InspectorDetail> for AltContent {
    fn from(detail: &InspectorDetail) -> Self {
        let name_text = detail.alt_display_name().map(str::to_owned);
        let description_lines = detail
            .alt_ubertip()
            .map(Description::lines_from)
            .unwrap_or_default();
        Self {
            name_text,
            description_lines,
        }
    }
}

/// Which of the override panel's editable fields are shown for this ability, given
/// the research-context flag and whether it has an off-state.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct FieldVisibility {
    pub(super) show_hotkey_field: bool,
    pub(super) show_research_field: bool,
    pub(super) show_alt_controls: bool,
    pub(super) upgrade_show: bool,
    pub(super) is_info_only: bool,
}

/// The inputs that decide [`FieldVisibility`]: the inspected ability, the
/// research-context flag, and its off-state content.
pub(super) struct FieldVisibilityInputs<'a> {
    pub(super) detail: &'a InspectorDetail,
    pub(super) is_research_context: bool,
    pub(super) alt_content: &'a AltContent,
}

impl From<FieldVisibilityInputs<'_>> for FieldVisibility {
    fn from(inputs: FieldVisibilityInputs<'_>) -> Self {
        let FieldVisibilityInputs {
            detail,
            is_research_context,
            alt_content,
        } = inputs;
        let show_hotkey_field =
            !detail.is_passive() && (!is_research_context || detail.is_command());
        let show_research_field =
            !detail.is_command() && is_research_context && !detail.info_only();
        let has_alt_state =
            alt_content.name_text.is_some() || !alt_content.description_lines.is_empty();
        let show_alt_controls = has_alt_state && !is_research_context && !detail.is_command();
        let upgrade_show = detail.upgrade_unit_id().is_some() && !is_research_context;
        let is_info_only = detail.info_only();
        Self {
            show_hotkey_field,
            show_research_field,
            show_alt_controls,
            upgrade_show,
            is_info_only,
        }
    }
}

/// The resolved upgrade-tier level: which of a multi-level ability's tiers is active,
/// its name, its description lines, and the "Level N of M" label.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct TierResolution {
    pub(super) total_tier_count: usize,
    pub(super) active_tier_index: usize,
    pub(super) active_tier_name: String,
    pub(super) description_lines: Vec<String>,
    pub(super) tier_label_text: String,
}

/// The inputs that resolve a [`TierResolution`]: the inspected ability, the stored
/// tier index, and the research-context flag.
pub(super) struct TierResolutionInputs<'a> {
    pub(super) detail: &'a InspectorDetail,
    pub(super) stored_tier_index: usize,
    pub(super) is_research_context: bool,
}

impl From<TierResolutionInputs<'_>> for TierResolution {
    fn from(inputs: TierResolutionInputs<'_>) -> Self {
        let TierResolutionInputs {
            detail,
            stored_tier_index,
            is_research_context,
        } = inputs;
        let ubertip_count = detail.ubertip_levels().len();
        let name_count = detail.name_levels().len();
        let icon_count = detail.icon_levels_len();
        let total_tier_count = ubertip_count.max(name_count).max(icon_count);
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
        let mut description_lines: Vec<String> = active_ubertip_text
            .as_deref()
            .map(Description::lines_from)
            .unwrap_or_default();
        if description_lines.is_empty() {
            let fallback_tip = if is_research_context {
                detail.research_tip().or(detail.tip())
            } else {
                detail.tip()
            };
            if let Some(text) = fallback_tip {
                description_lines = Tip::lines_from(text);
            }
        }
        let tier_label_text = format!("Level {} of {}", active_tier_index + 1, total_tier_count);
        Self {
            total_tier_count,
            active_tier_index,
            active_tier_name,
            description_lines,
            tier_label_text,
        }
    }
}

/// The presentation of a single editable hotkey field: the label to show (an en dash
/// when unbound), whether it is a special non-letter token, and whether it is the one
/// currently being edited.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct HotkeyFieldView {
    pub(super) label: String,
    pub(super) is_special: bool,
    pub(super) is_editing: bool,
}

impl HotkeyFieldView {
    pub(super) fn new(token: Option<HotkeyToken>, is_editing: bool) -> Self {
        let display = token.map(|token| token.display_label()).unwrap_or_default();
        let label = if display.is_empty() {
            String::from("\u{2013}")
        } else {
            display
        };
        let is_special = token
            .map(|token| char::try_from(token).is_err())
            .unwrap_or(false);
        Self {
            label,
            is_special,
            is_editing,
        }
    }
}

/// Which target the open picker is editing, resolved from the editing state: whether
/// it is open, the research flag, the currently-bound token, the object the pick
/// writes to (the upgraded unit for an upgrade edit), and the dialog title.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct PickerTarget {
    pub(super) open: bool,
    pub(super) is_research_context: bool,
    pub(super) current_token: Option<HotkeyToken>,
    pub(super) effective_object_id: WarcraftObjectId,
    pub(super) title: String,
}

/// The inputs that resolve the [`PickerTarget`]: the editing snapshot, the ability's
/// resolved tokens, the object the pick writes to, and its upgrade-form unit id.
pub(super) struct PickerTargetInputs<'a> {
    pub(super) snapshot: Option<OverrideEditTarget>,
    pub(super) tokens: &'a OverrideTokens,
    pub(super) object_id: WarcraftObjectId,
    pub(super) upgrade_unit_id: Option<WarcraftObjectId>,
}

impl From<PickerTargetInputs<'_>> for PickerTarget {
    fn from(inputs: PickerTargetInputs<'_>) -> Self {
        let PickerTargetInputs {
            snapshot,
            tokens,
            object_id,
            upgrade_unit_id,
        } = inputs;
        let open = snapshot.is_some();
        let is_research_context = matches!(snapshot, Some(OverrideEditTarget::ResearchHotkey));
        let current_token = match snapshot {
            Some(OverrideEditTarget::Hotkey) => tokens.hotkey,
            Some(OverrideEditTarget::ResearchHotkey) => tokens.research,
            Some(OverrideEditTarget::AltHotkey) => tokens.alt,
            Some(OverrideEditTarget::UpgradeHotkey) => tokens.upgrade,
            None => None,
        };
        let is_upgrade = matches!(snapshot, Some(OverrideEditTarget::UpgradeHotkey));
        let effective_object_id = if is_upgrade {
            upgrade_unit_id.unwrap_or(object_id)
        } else {
            object_id
        };
        let title = match snapshot {
            Some(OverrideEditTarget::ResearchHotkey) => String::from("Pick a research hotkey"),
            _ => String::from("Pick a hotkey"),
        };
        Self {
            open,
            is_research_context,
            current_token,
            effective_object_id,
            title,
        }
    }
}

/// The inputs a [`PickerBoard`] needs to resolve each key's availability.
#[derive(Clone, PartialEq, Debug)]
pub(super) struct PickerContext {
    pub(super) layout: GridLayout,
    pub(super) container_slots: Rc<[GridSlotId]>,
    pub(super) target_object_id: WarcraftObjectId,
    pub(super) current_token: Option<HotkeyToken>,
    pub(super) is_research_context: bool,
}

/// The keyboard laid out as picker cells, each marked current / conflicting /
/// available. Replaces the old fieldless `PickerRows` namespace.
#[derive(Clone, PartialEq)]
pub(super) struct PickerBoard {
    pub(super) rows: Vec<Vec<KeyPickerCell>>,
}

impl PickerBoard {
    pub(super) fn build(context: &PickerContext, custom_keys: Option<&CustomKeys>) -> Self {
        let container = &context.container_slots;
        let layout = context.layout;
        let is_research_context = context.is_research_context;
        let target = context.target_object_id;
        let rows = PICKER_ROWS
            .iter()
            .map(|row| {
                row.iter()
                    .map(|token| {
                        let token_value = *token;
                        let state = if Some(token_value) == context.current_token {
                            KeyPickerCellState::Current
                        } else if let Some(conflict) = hotkey_override::detect_conflict(
                            container,
                            target,
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
            .collect();
        Self { rows }
    }

    pub(super) fn into_rows(self) -> Vec<Vec<KeyPickerCell>> {
        self.rows
    }
}

/// The slot list a position picker offers: the edited slot pinned first, then every
/// container slot except the ability's own current entry.
#[derive(Clone, PartialEq, Debug, Default)]
pub(super) struct OverridePickerSlots {
    pub(super) slots: Rc<[GridSlotId]>,
}

impl OverridePickerSlots {
    pub(super) fn alt(object_id: WarcraftObjectId, container: &[GridSlotId]) -> Self {
        let mut combined: Vec<GridSlotId> = Vec::with_capacity(container.len() + 1);
        let off_state_slot = GridSlotId::ability_off(object_id);
        combined.push(off_state_slot);
        for slot in container.iter() {
            if let GridSlotId::Ability(ability_id) = slot
                && ability_id.object_id() == object_id
            {
                continue;
            }
            combined.push(*slot);
        }
        let slots = Rc::from(combined);
        Self { slots }
    }

    pub(super) fn upgrade(
        upgrade_id: WarcraftObjectId,
        base_object_id: WarcraftObjectId,
        container: &[GridSlotId],
    ) -> Self {
        let mut combined: Vec<GridSlotId> = Vec::with_capacity(container.len() + 1);
        let upgrade_slot = GridSlotId::ability(upgrade_id);
        combined.push(upgrade_slot);
        for slot in container.iter() {
            if let GridSlotId::Ability(base_id) = slot
                && base_id.object_id() == base_object_id
            {
                continue;
            }
            combined.push(*slot);
        }
        let slots = Rc::from(combined);
        Self { slots }
    }

    pub(super) fn into_slots(self) -> Rc<[GridSlotId]> {
        self.slots
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

fn use_override_editing(
    props: &HotkeyOverrideModel,
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
        let conflict = hotkey_override::detect_conflict(
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
    let on_hotkey_alt_position_click = EventHandler::new(move |_: ()| alt_open_for_click.set(true));
    let mut upgrade_open_for_click = upgrade_open;
    let on_hotkey_upgrade_position_click =
        EventHandler::new(move |_: ()| upgrade_open_for_click.set(true));
    PositionPickers {
        alt_open,
        upgrade_open,
        on_hotkey_alt_position_click,
        on_hotkey_upgrade_position_click,
    }
}

pub(super) fn use_hotkey_override(props: &HotkeyOverrideModel) -> HotkeyOverridePresentation {
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

    let inputs = HotkeyOverrideInputs {
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
    HotkeyOverridePresentation::from(inputs)
}
