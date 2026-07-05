use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::text::description::Description;
use warcraft_keybinds::text::tip::Tip;
use warcraft_keybinds::{CustomKeys, GridLayout, GridSlotId, HotkeyToken, InspectorDetail, Letter};

use super::state::OverrideEditTarget;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker::{KeyPickerCell, KeyPickerCellState};
use crate::services::customkeys::hotkey_override::HotkeyOverride;

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

impl OverrideTokens {
    pub(super) fn resolve(detail: &InspectorDetail, layout: GridLayout) -> Self {
        let layout_hotkey = detail
            .button_position()
            .and_then(|position| {
                let column = position.column();
                let row = position.row();
                layout.letter_at(column, row)
            })
            .and_then(|character| HotkeyToken::try_from(character).ok());
        let research_position = detail
            .research_button_position()
            .or(detail.button_position());
        let layout_research = research_position
            .and_then(|position| {
                let column = position.column();
                let row = position.row();
                layout.letter_at(column, row)
            })
            .and_then(|character| HotkeyToken::try_from(character).ok());
        let hotkey = detail.hotkey_token().or(layout_hotkey);
        let research = detail.research_hotkey_token().or(layout_research);
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

impl FieldVisibility {
    pub(super) fn resolve(
        detail: &InspectorDetail,
        is_research_context: bool,
        alt_content: &AltContent,
    ) -> Self {
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

impl TierResolution {
    pub(super) fn resolve(
        detail: &InspectorDetail,
        stored_tier_index: usize,
        is_research_context: bool,
    ) -> Self {
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

impl PickerTarget {
    pub(super) fn resolve(
        snapshot: Option<OverrideEditTarget>,
        tokens: &OverrideTokens,
        object_id: WarcraftObjectId,
        upgrade_unit_id: Option<WarcraftObjectId>,
    ) -> Self {
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
        let target = context.target_object_id.value();
        let rows = PICKER_ROWS
            .iter()
            .map(|row| {
                row.iter()
                    .map(|token| {
                        let token_value = *token;
                        let state = if Some(token_value) == context.current_token {
                            KeyPickerCellState::Current
                        } else if let Some(conflict) = HotkeyOverride::detect_conflict(
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
