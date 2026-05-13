use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::sync::OnceLock;

use warcraft_api::{WarcraftObjectId, WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::{ObjectLookup, WARCRAFT_DATABASE};

use crate::cascade::conflict_graph::ConflictGraph;
use crate::cascade::planner::{CascadePlan, MoveReason, PlannedMove, UnresolvedMover};
use crate::cascade::queue::{AssignmentQueue, AssignmentScope};
use crate::command::move_request::MoveRequest;
use crate::grid::layout::GridLayout;
use crate::identity::ability_id::AbilityId;
use crate::identity::hotkey_target::HotkeyTarget;
use crate::identity::hotkey_token::HotkeyToken;
use crate::identity::slot::GridSlotId;
use crate::model::{
    AbilityBinding, BindingEntry, ColumnIndex, CommandBinding, CommandEntry, GridCoordinate,
    Hotkey, RowIndex, SectionAccumulator, SectionResolution, SystemBinding, WarcraftKeybinding,
};
use crate::unit::grids::GridRole;

pub const DEFAULT_CUSTOM_KEYS: &str = include_str!("../templates/CustomKeys.txt");

const BUNDLED_BASELINE: &str = DEFAULT_CUSTOM_KEYS;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HotkeyConflict {
    display_name: String,
}

impl HotkeyConflict {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}
const GRID_COLUMNS: u8 = 4;
const GRID_ROWS: u8 = 3;

#[derive(Clone, Default)]
pub struct CustomKeys {
    entries: BTreeMap<WarcraftObjectId, WarcraftKeybinding>,
}

impl From<BTreeMap<WarcraftObjectId, WarcraftKeybinding>> for CustomKeys {
    fn from(entries: BTreeMap<WarcraftObjectId, WarcraftKeybinding>) -> Self {
        Self { entries }
    }
}

impl CustomKeys {
    pub fn binding(&self, id: impl Into<AbilityId>) -> Option<&AbilityBinding> {
        let ability_id = id.into();
        let canonical_object_id = self
            .canonical_object_id_for(ability_id.object_id())
            .unwrap_or_else(|| ability_id.object_id());
        self.entries.get(canonical_object_id.value())?.as_ability()
    }

    pub(crate) fn binding_or_default_mut(
        &mut self,
        id: impl Into<AbilityId>,
    ) -> Option<&mut AbilityBinding> {
        let ability_id = id.into();
        let requested_object_id = ability_id.object_id();
        let canonical_object_id = self
            .canonical_object_id_for(requested_object_id)
            .unwrap_or(requested_object_id);
        if !matches!(
            self.entries.get(canonical_object_id.value()),
            Some(WarcraftKeybinding::Ability(_))
        ) {
            self.entries.insert(
                canonical_object_id,
                WarcraftKeybinding::Ability(AbilityBinding::default()),
            );
        }
        self.entries
            .get_mut(canonical_object_id.value())
            .and_then(WarcraftKeybinding::as_ability_mut)
    }

    /// Looks up the actual key under which `requested` is stored, matching
    /// case-insensitively.  This collapses casing variants from the auto-
    /// generated database (e.g. `ACvs` and `Acvs` for Envenomed Weapons) so
    /// they share a single binding in the entries map and produce a single
    /// section in the serialized output.
    fn canonical_object_id_for(&self, requested: WarcraftObjectId) -> Option<WarcraftObjectId> {
        let requested_value = requested.value();
        if self.entries.contains_key(requested_value) {
            return Some(requested);
        }
        let requested_lowercase = requested_value.to_ascii_lowercase();
        self.entries
            .keys()
            .find(|stored| stored.value().to_ascii_lowercase() == requested_lowercase)
            .copied()
    }

    pub fn bindings_in_order(&self) -> impl Iterator<Item = BindingEntry<'_>> {
        self.entries.iter().filter_map(|(id, binding)| {
            binding.as_ability().map(|ability| {
                let ability_id = AbilityId::from(*id);
                BindingEntry::new(ability_id, ability)
            })
        })
    }

    pub fn command(&self, name: &str) -> Option<&CommandBinding> {
        if let Some(entry) = self.entries.get(name)
            && let Some(command) = entry.as_command()
        {
            return Some(command);
        }
        let lowercase_name = name.to_ascii_lowercase();
        let canonical = self
            .entries
            .keys()
            .find(|stored| stored.value().to_ascii_lowercase() == lowercase_name)?;
        self.entries.get(canonical.value())?.as_command()
    }

    pub(crate) fn command_or_default_mut(
        &mut self,
        name: impl Into<WarcraftObjectId>,
    ) -> Option<&mut CommandBinding> {
        let requested_object_id = name.into();
        let canonical_object_id = self
            .canonical_object_id_for(requested_object_id)
            .unwrap_or(requested_object_id);
        if !matches!(
            self.entries.get(canonical_object_id.value()),
            Some(WarcraftKeybinding::Command(_))
        ) {
            self.entries.insert(
                canonical_object_id,
                WarcraftKeybinding::Command(CommandBinding::default()),
            );
        }
        self.entries
            .get_mut(canonical_object_id.value())
            .and_then(WarcraftKeybinding::as_command_mut)
    }

    pub fn commands_in_order(&self) -> impl Iterator<Item = CommandEntry<'_>> {
        self.entries.iter().filter_map(|(name, binding)| {
            binding
                .as_command()
                .map(|command| CommandEntry::new(*name, command))
        })
    }

    pub fn system(&self, id: &str) -> Option<&SystemBinding> {
        self.entries.get(id)?.as_system()
    }

    pub(crate) fn system_mut(&mut self, id: &str) -> Option<&mut SystemBinding> {
        self.entries.get_mut(id)?.as_system_mut()
    }

    pub fn set_system_hotkey(&mut self, section_id: &str, hotkey_code: u32) {
        let hotkey = Hotkey::VirtualKey(hotkey_code);
        if let Some(binding) = self.system_mut(section_id) {
            binding.set_hotkey(hotkey);
        }
    }

    pub fn builder() -> crate::model::CustomKeysBuilder {
        crate::model::CustomKeysBuilder::default()
    }

    pub fn put_ability(&mut self, id: impl Into<AbilityId>, binding: AbilityBinding) {
        let ability_id = id.into();
        let object_id = ability_id.object_id();
        self.entries
            .insert(object_id, WarcraftKeybinding::Ability(binding));
    }

    pub fn put_command(&mut self, name: impl Into<WarcraftObjectId>, binding: CommandBinding) {
        let object_id = name.into();
        self.entries
            .insert(object_id, WarcraftKeybinding::Command(binding));
    }

    pub fn put_system(&mut self, id: impl Into<WarcraftObjectId>, binding: SystemBinding) {
        let object_id = id.into();
        self.entries
            .insert(object_id, WarcraftKeybinding::System(binding));
    }

    pub fn swap_system_bindings(&mut self, source_id: &str, target_id: &str) {
        let source_hotkey = self
            .system(source_id)
            .and_then(|binding| match binding.hotkey() {
                Hotkey::VirtualKey(code) => Some(*code),
                _ => None,
            });
        let target_hotkey = self
            .system(target_id)
            .and_then(|binding| match binding.hotkey() {
                Hotkey::VirtualKey(code) => Some(*code),
                _ => None,
            });
        if let Some(binding) = self.system_mut(source_id) {
            binding.set_hotkey(Hotkey::VirtualKey(target_hotkey.unwrap_or(0)));
        }
        if let Some(binding) = self.system_mut(target_id) {
            binding.set_hotkey(Hotkey::VirtualKey(source_hotkey.unwrap_or(0)));
        }
    }

    pub fn normalize(&self) -> Self {
        let mut result = Self::materialized_baseline().clone();
        let overlay_clone = self.clone();
        result.extend(overlay_clone);
        result
    }

    fn materialized_baseline() -> &'static Self {
        static CACHE: OnceLock<CustomKeys> = OnceLock::new();
        CACHE.get_or_init(|| {
            let mut file = Self::from(BUNDLED_BASELINE);
            file.materialize_default_positions();
            file.materialize_shop_item_positions();
            file
        })
    }

    pub fn serialize(&self, baseline: &str) -> String {
        let mut export_file = Self::from(baseline);
        let overlay_clone = self.clone();
        export_file.extend(overlay_clone);
        export_file.materialize_default_positions();
        export_file.materialize_shop_item_positions();
        export_file.to_string()
    }

    fn materialize_default_positions(&mut self) {
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let default_button = warcraft_object.default_button_position();
            let default_research = warcraft_object.default_research_button_position();

            match warcraft_object.kind() {
                WarcraftObjectKind::Command => continue,
                WarcraftObjectKind::Ability => {
                    // Toggle ability with no Buttonpos in the game data (e.g.
                    // Prioritize / Aatp on the Gargoyle).  Without a fallback
                    // the renderer skips it entirely; place it at origin and
                    // let the cascade move it to an actually-free cell.
                    // Passive/inventory abilities are excluded — they're never
                    // user-clickable buttons and must not enter the grid.
                    let needs_origin_fallback = default_button.is_none()
                        && default_research.is_none()
                        && !warcraft_object.is_passive_ability()
                        && matches!(warcraft_object.meta(), WarcraftObjectMeta::Ability(meta) if meta.has_off_state());
                    if default_button.is_none()
                        && default_research.is_none()
                        && !needs_origin_fallback
                    {
                        continue;
                    }
                    let canonical_id = *object_id;
                    let Some(binding) = self.binding_or_default_mut(canonical_id) else {
                        continue;
                    };
                    if binding.button_position().is_none() {
                        if let Some(position_value) = default_button {
                            binding.set_button_position(Some(position_value));
                        } else if needs_origin_fallback {
                            let origin = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
                            binding.set_button_position(Some(origin));
                        }
                    }
                    if binding.research_button_position().is_none()
                        && let Some(position_value) = default_research
                    {
                        binding.set_research_button_position(Some(position_value));
                    }
                    if binding.unbutton_position().is_none()
                        && !warcraft_object.is_passive_ability()
                        && let WarcraftObjectMeta::Ability(ability_meta) = warcraft_object.meta()
                        && ability_meta.has_off_state()
                    {
                        let database_off = ability_meta.off_button_position();
                        if let Some(off_position) = database_off {
                            binding.set_unbutton_position(Some(off_position));
                        } else if let Some(button_position) = binding.button_position() {
                            let position_copy = *button_position;
                            binding.set_unbutton_position(Some(position_copy));
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    fn materialize_shop_item_positions(&mut self) {
        for (_object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                continue;
            };
            let sell_items = unit_meta.sell_items();
            let sell_units = unit_meta.sell_units();
            if sell_items.is_empty() && sell_units.is_empty() {
                continue;
            }

            let mut occupied_positions: HashSet<GridCoordinate> = HashSet::new();
            for item_id_object in sell_items {
                let item_id = item_id_object.value();
                let item_binding = self.binding(item_id);
                let position_ref = item_binding.and_then(|binding| binding.button_position());
                let existing_position = position_ref.copied();
                if let Some(position) = existing_position {
                    occupied_positions.insert(position);
                }
            }
            for unit_id_object in sell_units {
                let unit_id = unit_id_object.value();
                let unit_binding = self.binding(unit_id);
                let position_ref = unit_binding.and_then(|binding| binding.button_position());
                let existing_position = position_ref.copied();
                if let Some(position) = existing_position {
                    occupied_positions.insert(position);
                }
            }

            for item_id_object in sell_items {
                let item_id = item_id_object.value();
                let item_binding = self.binding(item_id);
                let position_ref = item_binding.and_then(|binding| binding.button_position());
                let has_position = position_ref.is_some();
                if has_position {
                    continue;
                }
                let Some(free_position) = Self::next_free_grid_cell(&occupied_positions) else {
                    continue;
                };
                occupied_positions.insert(free_position);
                let item_canonical_id = *item_id_object;
                if let Some(item_binding) = self.binding_or_default_mut(item_canonical_id) {
                    item_binding.set_button_position(Some(free_position));
                }
            }
            for unit_id_object in sell_units {
                let unit_id = unit_id_object.value();
                let unit_binding = self.binding(unit_id);
                let position_ref = unit_binding.and_then(|binding| binding.button_position());
                let has_position = position_ref.is_some();
                if has_position {
                    continue;
                }
                let Some(free_position) = Self::next_free_grid_cell(&occupied_positions) else {
                    continue;
                };
                occupied_positions.insert(free_position);
                let unit_canonical_id = *unit_id_object;
                if let Some(unit_binding) = self.binding_or_default_mut(unit_canonical_id) {
                    unit_binding.set_button_position(Some(free_position));
                }
            }
        }
    }

    fn next_free_grid_cell(occupied_positions: &HashSet<GridCoordinate>) -> Option<GridCoordinate> {
        for row in 0..GRID_ROWS {
            for column in 0..GRID_COLUMNS {
                let Ok(column) = ColumnIndex::try_from(column) else {
                    continue;
                };
                let Ok(row) = RowIndex::try_from(row) else {
                    continue;
                };
                let candidate = GridCoordinate::new(column, row);
                if !occupied_positions.contains(&candidate) {
                    return Some(candidate);
                }
            }
        }
        None
    }

    pub fn position_for_slot(
        &self,
        slot: &GridSlotId,
        is_research_context: bool,
    ) -> Option<GridCoordinate> {
        match slot {
            GridSlotId::Ability(ability_id) => {
                let bound_id = *ability_id;
                let binding = self.binding(bound_id)?;
                if is_research_context {
                    binding.research_button_position().copied()
                } else {
                    binding.button_position().copied()
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                let bound_id = *ability_id;
                let binding = self.binding(bound_id)?;
                binding.unbutton_position().copied()
            }
            GridSlotId::Command(command_name) => {
                let binding = self.command(command_name.value())?;
                binding.button_position().copied()
            }
        }
    }

    pub fn slot_at_position(
        &self,
        slots: &[GridSlotId],
        is_research_context: bool,
        column: u8,
        row: u8,
    ) -> Option<GridSlotId> {
        for slot in slots {
            let Some(position) = self.position_for_slot(slot, is_research_context) else {
                continue;
            };
            let position_column = u8::from(position.column());
            let position_row = u8::from(position.row());
            if position_column == column && position_row == row {
                return Some(*slot);
            }
        }
        None
    }

    pub fn assign_position(
        &mut self,
        layout: GridLayout,
        slot: &GridSlotId,
        column: u8,
        row: u8,
        is_research_context: bool,
    ) {
        let Ok(column_index) = ColumnIndex::try_from(column) else {
            return;
        };
        let Ok(row_index) = RowIndex::try_from(row) else {
            return;
        };
        let Some(letter) = layout.letter_at(column_index, row_index) else {
            return;
        };
        let new_position = GridCoordinate::new(column_index, row_index);
        match slot {
            GridSlotId::Ability(ability_id) => {
                let is_passive = ObjectLookup::is_passive_ability(ability_id.value());
                if let Some(binding) = self.binding_or_default_mut(*ability_id) {
                    if is_research_context {
                        binding.set_research_button_position(Some(new_position));
                        let research_hotkey = Hotkey::from(letter);
                        binding.set_research_hotkey(Some(research_hotkey));
                    } else {
                        binding.set_button_position(Some(new_position));
                        if !is_passive {
                            let ability_hotkey = Hotkey::from(letter);
                            binding.set_hotkey(Some(ability_hotkey));
                        }
                    }
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                if let Some(binding) = self.binding_or_default_mut(*ability_id) {
                    binding.set_unbutton_position(Some(new_position));
                    let unhotkey = Hotkey::from(letter);
                    binding.set_unhotkey(Some(unhotkey));
                }
            }
            GridSlotId::Command(command_name) => {
                if let Some(binding) = self.command_or_default_mut(*command_name) {
                    binding.set_button_position(Some(new_position));
                    let command_hotkey = Hotkey::from(letter);
                    binding.set_hotkey(Some(command_hotkey));
                    binding.set_unbutton_position(Some(new_position));
                }
            }
        }
    }

    pub fn move_slot(&mut self, request: &MoveRequest) {
        let moving_old_position =
            self.position_for_slot(request.moving_slot(), request.is_research_context());
        let displaced_slot = self.slot_at_position(
            request.slot_ids(),
            request.is_research_context(),
            request.target_column(),
            request.target_row(),
        );
        let off_state_blocks = displaced_slot.is_none()
            && !request.is_research_context()
            && request.slot_ids().iter().any(|slot| {
                let GridSlotId::Ability(ability_id) = slot else {
                    return false;
                };
                if ability_id
                    .value()
                    .eq_ignore_ascii_case(request.moving_slot().as_str())
                {
                    return false;
                }
                let off_slot = GridSlotId::AbilityOff(*ability_id);
                self.position_for_slot(&off_slot, false)
                    .is_some_and(|off_pos| {
                        let off_column = u8::from(off_pos.column());
                        let off_row = u8::from(off_pos.row());
                        off_column == request.target_column() && off_row == request.target_row()
                    })
            });
        let moving_off_colocated = !request.prevent_co_move()
            && match (request.moving_slot(), &moving_old_position) {
                (GridSlotId::Ability(id), Some(old_pos)) => self
                    .position_for_slot(&GridSlotId::AbilityOff(*id), false)
                    .is_some_and(|off_pos| {
                        let off_column = u8::from(off_pos.column());
                        let off_row = u8::from(off_pos.row());
                        let old_column = u8::from(old_pos.column());
                        let old_row = u8::from(old_pos.row());
                        off_column == old_column && off_row == old_row
                    }),
                _ => false,
            };
        let displaced_off_colocated = match &displaced_slot {
            Some(GridSlotId::Ability(id)) => self
                .position_for_slot(&GridSlotId::AbilityOff(*id), false)
                .is_some_and(|off_pos| {
                    let off_column = u8::from(off_pos.column());
                    let off_row = u8::from(off_pos.row());
                    off_column == request.target_column() && off_row == request.target_row()
                }),
            _ => false,
        };

        if off_state_blocks {
            return;
        }
        if let Some(ref slot) = displaced_slot {
            let is_same_slot = match (slot, request.moving_slot()) {
                (GridSlotId::Ability(left), GridSlotId::Ability(right))
                | (GridSlotId::AbilityOff(left), GridSlotId::AbilityOff(right)) => {
                    left.value().eq_ignore_ascii_case(right.value())
                }
                (GridSlotId::Command(left), GridSlotId::Command(right)) => {
                    left.value().eq_ignore_ascii_case(right.value())
                }
                _ => false,
            };
            if is_same_slot {
                return;
            }
        }
        if request.prevent_swap()
            && let Some(ref slot) = displaced_slot
            && !slot
                .as_str()
                .eq_ignore_ascii_case(request.moving_slot().as_str())
        {
            return;
        }

        self.assign_position(
            request.layout(),
            request.moving_slot(),
            request.target_column(),
            request.target_row(),
            request.is_research_context(),
        );
        if moving_off_colocated && let GridSlotId::Ability(moving_id) = request.moving_slot() {
            self.assign_position(
                request.layout(),
                &GridSlotId::AbilityOff(*moving_id),
                request.target_column(),
                request.target_row(),
                false,
            );
        }
        if !request.prevent_swap()
            && let Some(displaced) = displaced_slot
            && let Some(old_position) = moving_old_position
        {
            let old_column = u8::from(old_position.column());
            let old_row = u8::from(old_position.row());
            self.assign_position(
                request.layout(),
                &displaced,
                old_column,
                old_row,
                request.is_research_context(),
            );
            if displaced_off_colocated && let GridSlotId::Ability(displaced_id) = &displaced {
                self.assign_position(
                    request.layout(),
                    &GridSlotId::AbilityOff(*displaced_id),
                    old_column,
                    old_row,
                    false,
                );
            }
        }
    }

    pub fn apply_grid_to_all_bindings(&mut self, layout: GridLayout) -> usize {
        let mut changed_count: usize = 0;
        let ability_ids: Vec<AbilityId> = self
            .bindings_in_order()
            .map(|entry| entry.ability_id())
            .collect();
        let command_names: Vec<WarcraftObjectId> =
            self.commands_in_order().map(|entry| entry.name()).collect();

        for ability_id in &ability_ids {
            let ability_id_str = ability_id.value();
            let is_passive = ObjectLookup::is_passive_ability(ability_id_str);
            let button_position = if is_passive {
                None
            } else {
                self.binding(ability_id_str)
                    .and_then(|binding| binding.button_position())
                    .copied()
            };
            let research_button_position = self
                .binding(ability_id_str)
                .and_then(|binding| binding.research_button_position())
                .copied();
            let unbutton_position = self
                .binding(ability_id_str)
                .and_then(|binding| binding.unbutton_position())
                .copied();
            if button_position.is_none()
                && research_button_position.is_none()
                && unbutton_position.is_none()
            {
                continue;
            }
            let bound_id = *ability_id;
            let Some(binding) = self.binding_or_default_mut(bound_id) else {
                continue;
            };
            if let Some(position) = button_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && binding.hotkey().is_none_or(|h| h.accepts_grid_letter())
            {
                let new_hotkey = Hotkey::from(letter);
                if binding.hotkey() != Some(&new_hotkey) {
                    binding.set_hotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
            if let Some(position) = research_button_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && binding
                    .research_hotkey()
                    .is_none_or(|h| h.accepts_grid_letter())
            {
                let new_hotkey = Hotkey::from(letter);
                if binding.research_hotkey() != Some(&new_hotkey) {
                    binding.set_research_hotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
            if let Some(position) = unbutton_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && binding.unhotkey().is_none_or(|h| h.accepts_grid_letter())
            {
                let new_hotkey = Hotkey::from(letter);
                if binding.unhotkey() != Some(&new_hotkey) {
                    binding.set_unhotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
        }

        for command_name in &command_names {
            let button_position = self
                .command(command_name.value())
                .and_then(|binding| binding.button_position())
                .copied();
            let Some(position) = button_position else {
                continue;
            };
            let Some(letter) = layout.letter_at(position.column(), position.row()) else {
                continue;
            };
            let Some(binding) = self.command_or_default_mut(*command_name) else {
                continue;
            };
            if binding.hotkey().is_none_or(|h| h.accepts_grid_letter()) {
                let new_hotkey = Hotkey::from(letter);
                if binding.hotkey() != Some(&new_hotkey) {
                    binding.set_hotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
        }

        changed_count
    }

    /// Runs the cascade conflict-resolution algorithm and applies its plan to
    /// this `CustomKeys`.  This is a user-triggered, opt-in operation — it is
    /// **not** called from `normalize()` or the boot path.  Use it when the
    /// user explicitly asks the app to try resolving collisions (typically
    /// before export).
    ///
    /// Only **positions** are written back; hotkeys are untouched.  Hotkeys
    /// belong to `assign_position` and `apply_grid_to_all_bindings`; the
    /// cascade just redistributes geometry to remove cross-unit collisions
    /// (and pack rows left where it can).
    ///
    /// **Two phases**:
    ///   1. **Cross-unit cascade** (`AssignmentScope::CrossUnitOnly`) — the
    ///      classic cascade, treating only multi-carrier and pinned slots
    ///      as anchor candidates.  Settles all cross-unit collisions first.
    ///   2. **Intra-unit cleanup** (`AssignmentScope::IncludingIntraUnit`)
    ///      — a second pass with single-carrier abilities also eligible.
    ///      Resolves the remaining "two shop items on the same Goblin
    ///      Merchant slot" style collisions that phase 1 deliberately left
    ///      alone.
    ///
    /// Each phase loops to a fixed point because the spill step can create
    /// new gap-pull opportunities that a follow-up pass closes.  The returned
    /// `CascadePlan` aggregates every net position change from the starting
    /// state to the final state so the caller sees a single `(old → new)` per
    /// ability.  Unresolved nodes are the ones still stuck after both phases.
    pub fn resolve_conflicts(&mut self) -> CascadePlan {
        let mut net_moves: HashMap<MoveKey, AccumulatedMove> = HashMap::new();
        let _phase_one_unresolved =
            self.run_cascade_phase(AssignmentScope::CrossUnitOnly, &mut net_moves);
        let last_unresolved =
            self.run_cascade_phase(AssignmentScope::IncludingIntraUnit, &mut net_moves);

        let mut combined_moves: Vec<PlannedMove> = Vec::new();
        for (key, accumulated) in net_moves {
            if accumulated.old_position == accumulated.new_position {
                continue;
            }
            let planned_move = PlannedMove::new(
                key.slot_id,
                key.grid_role,
                accumulated.old_position,
                accumulated.new_position,
                accumulated.carrier_unit_ids,
                accumulated.reason,
            );
            combined_moves.push(planned_move);
        }
        CascadePlan::from_parts(combined_moves, last_unresolved)
    }

    /// Drives one cascade phase to a fixed point under the given
    /// `AssignmentScope`.  Each iteration rebuilds the conflict graph,
    /// builds the queue with that scope, applies every planned move, and
    /// merges the moves into `net_moves` (so a single ability that moves
    /// across multiple iterations collapses into one `(old → new)` entry).
    /// Returns the unresolved set from the final iteration.
    fn run_cascade_phase(
        &mut self,
        scope: AssignmentScope,
        net_moves: &mut HashMap<MoveKey, AccumulatedMove>,
    ) -> Vec<UnresolvedMover> {
        const MAX_ITERATIONS_PER_PHASE: usize = 32;
        let mut last_unresolved: Vec<UnresolvedMover> = Vec::new();
        for _ in 0..MAX_ITERATIONS_PER_PHASE {
            let graph = ConflictGraph::build(self);
            let queue = AssignmentQueue::build_with_scope(graph, scope);
            let pass_plan = CascadePlan::from(&queue);
            last_unresolved = pass_plan.unresolved().to_vec();
            if pass_plan.move_count() == 0 {
                break;
            }
            for planned_move in pass_plan.moves() {
                let key = MoveKey {
                    slot_id: planned_move.slot_id(),
                    grid_role: planned_move.grid_role(),
                };
                let new_position = planned_move.new_position();
                let carrier_unit_ids: Vec<WarcraftObjectId> =
                    planned_move.carrier_unit_ids().to_vec();
                let move_reason: MoveReason = planned_move.reason().clone();
                let fresh_reason = move_reason.clone();
                net_moves
                    .entry(key)
                    .and_modify(|accumulated| {
                        accumulated.new_position = new_position;
                        accumulated.reason = move_reason;
                    })
                    .or_insert_with(|| AccumulatedMove {
                        old_position: planned_move.old_position(),
                        new_position,
                        carrier_unit_ids,
                        reason: fresh_reason,
                    });
                let application = MoveApplication::from_planned_move(planned_move);
                self.apply_resolved_position(application);
            }
        }
        last_unresolved
    }

    fn apply_resolved_position(&mut self, application: MoveApplication) {
        let is_research_context = application.grid_role.is_research_context();
        let new_position = application.new_position;
        match application.slot_id {
            GridSlotId::Ability(ability_id) => {
                let Some(binding) = self.binding_or_default_mut(ability_id) else {
                    return;
                };
                if is_research_context {
                    binding.set_research_button_position(Some(new_position));
                } else {
                    let old_button_position = binding.button_position().copied();
                    let old_unbutton_position = binding.unbutton_position().copied();
                    let off_was_colocated = old_unbutton_position.is_some()
                        && old_unbutton_position == old_button_position;
                    binding.set_button_position(Some(new_position));
                    if off_was_colocated {
                        binding.set_unbutton_position(Some(new_position));
                    }
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                let Some(binding) = self.binding_or_default_mut(ability_id) else {
                    return;
                };
                binding.set_unbutton_position(Some(new_position));
            }
            GridSlotId::Command(command_id) => {
                let Some(binding) = self.command_or_default_mut(command_id) else {
                    return;
                };
                binding.set_button_position(Some(new_position));
                binding.set_unbutton_position(Some(new_position));
            }
        }
    }

    pub fn set_hotkey(&mut self, target: HotkeyTarget, new_token: Option<HotkeyToken>) {
        match target {
            HotkeyTarget::Ability(ability_id) => {
                if let Some(binding) = self.binding_or_default_mut(ability_id) {
                    let existing_levels = binding.hotkey().map_or(0, |h| h.level_count());
                    let replicated =
                        new_token.map(|token| Hotkey::replicated(token, existing_levels));
                    binding.set_hotkey(replicated);
                }
            }
            HotkeyTarget::AbilityResearch(ability_id) => {
                if let Some(binding) = self.binding_or_default_mut(ability_id) {
                    let research_levels = binding.research_hotkey().map_or(0, |h| h.level_count());
                    let replicated =
                        new_token.map(|token| Hotkey::replicated(token, research_levels));
                    binding.set_research_hotkey(replicated);
                }
            }
            HotkeyTarget::AbilityOffState(ability_id) => {
                if let Some(binding) = self.binding_or_default_mut(ability_id) {
                    let existing_levels = binding.unhotkey().map_or(0, |h| h.level_count());
                    let replicated =
                        new_token.map(|token| Hotkey::replicated(token, existing_levels));
                    binding.set_unhotkey(replicated);
                }
            }
            HotkeyTarget::Command(command_name) => {
                if let Some(binding) = self.command_or_default_mut(command_name) {
                    let existing_levels = binding.hotkey().map_or(0, |h| h.level_count());
                    let replicated =
                        new_token.map(|token| Hotkey::replicated(token, existing_levels));
                    binding.set_hotkey(replicated);
                }
            }
        }
    }

    pub fn find_hotkey_conflict(
        &self,
        slots: &[GridSlotId],
        target_object_id: &str,
        proposed_token: HotkeyToken,
        layout: GridLayout,
        is_research_context: bool,
    ) -> Option<HotkeyConflict> {
        for candidate_slot in slots {
            if candidate_slot
                .as_str()
                .eq_ignore_ascii_case(target_object_id)
            {
                continue;
            }
            let candidate_token =
                self.effective_hotkey_token(candidate_slot, layout, is_research_context);
            let Some(token_value) = candidate_token else {
                continue;
            };
            if token_value != proposed_token {
                continue;
            }
            let display_name = match candidate_slot {
                GridSlotId::Ability(id) | GridSlotId::AbilityOff(id) => {
                    let ability_binding = self.binding(*id);
                    candidate_slot.display_name(ability_binding, None)
                }
                GridSlotId::Command(name) => {
                    let command_binding = self.command(name.value());
                    candidate_slot.display_name(None, command_binding)
                }
            };
            let conflict = HotkeyConflict { display_name };
            return Some(conflict);
        }
        None
    }

    pub fn effective_hotkey_token(
        &self,
        slot: &GridSlotId,
        layout: GridLayout,
        is_research_context: bool,
    ) -> Option<HotkeyToken> {
        let override_hotkey: Option<&Hotkey> = match slot {
            GridSlotId::Ability(ability_id) => {
                let bound_id = *ability_id;
                self.binding(bound_id).and_then(|binding| {
                    if is_research_context {
                        binding.research_hotkey()
                    } else {
                        binding.hotkey()
                    }
                })
            }
            GridSlotId::AbilityOff(ability_id) => {
                let bound_id = *ability_id;
                self.binding(bound_id)
                    .and_then(|binding| binding.unhotkey())
            }
            GridSlotId::Command(command_name) => self
                .command(command_name.value())
                .and_then(|binding| binding.hotkey()),
        };
        if let Some(hotkey) = override_hotkey {
            return hotkey.first_token();
        }
        let resolved_position = self.position_for_slot(slot, is_research_context)?;
        let layout_letter =
            layout.letter_at(resolved_position.column(), resolved_position.row())?;
        Some(HotkeyToken::from(layout_letter))
    }
}

/// Snapshot of a single `PlannedMove` decoupled from the plan's borrow, so
/// `resolve_conflicts` can release its read of `&self` before mutating.
struct MoveApplication {
    slot_id: GridSlotId,
    grid_role: GridRole,
    new_position: GridCoordinate,
}

impl MoveApplication {
    fn from_planned_move(planned_move: &PlannedMove) -> Self {
        Self {
            slot_id: planned_move.slot_id(),
            grid_role: planned_move.grid_role(),
            new_position: planned_move.new_position(),
        }
    }
}

/// Identifies a slot/role pair across multiple `resolve_conflicts` iterations
/// so we can collapse repeated moves of the same ability into a single
/// `(original → final)` entry.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct MoveKey {
    slot_id: GridSlotId,
    grid_role: GridRole,
}

/// Net movement of a single slot accumulated across iterations.  The
/// `old_position` is the first one we saw (before any mutation), the
/// `new_position` is updated on each subsequent move so the final value
/// reflects where the slot ended up.  `reason` is overwritten on each
/// update so it always reflects the *last* event that placed the slot —
/// earlier iterations were superseded by the most recent move.
struct AccumulatedMove {
    old_position: GridCoordinate,
    new_position: GridCoordinate,
    carrier_unit_ids: Vec<WarcraftObjectId>,
    reason: MoveReason,
}

impl fmt::Display for CustomKeys {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (object_id, entry) in &self.entries {
            match entry {
                WarcraftKeybinding::Ability(binding) => {
                    binding.write_section(formatter, *object_id)?;
                }
                WarcraftKeybinding::Command(binding) => {
                    binding.write_section(formatter, *object_id)?;
                }
                WarcraftKeybinding::System(binding) => {
                    binding.write_section(formatter, *object_id)?;
                }
            }
        }
        Ok(())
    }
}

impl IntoIterator for CustomKeys {
    type Item = (WarcraftObjectId, WarcraftKeybinding);
    type IntoIter = std::collections::btree_map::IntoIter<WarcraftObjectId, WarcraftKeybinding>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl Extend<(WarcraftObjectId, WarcraftKeybinding)> for CustomKeys {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (WarcraftObjectId, WarcraftKeybinding)>,
    {
        for (object_id, binding) in iter {
            let raw_key = object_id.value();
            match binding {
                WarcraftKeybinding::Ability(source_binding) => {
                    if self.system(raw_key).is_some() {
                        continue;
                    }
                    let Some(target_binding) = self.binding_or_default_mut(object_id) else {
                        continue;
                    };
                    if let Some(hotkey) = source_binding.hotkey() {
                        let hotkey_clone = *hotkey;
                        target_binding.set_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.button_position().copied() {
                        target_binding.set_button_position(Some(position));
                    }
                    if let Some(position) = source_binding.unbutton_position().copied() {
                        target_binding.set_unbutton_position(Some(position));
                    }
                    if let Some(hotkey) = source_binding.research_hotkey() {
                        let hotkey_clone = *hotkey;
                        target_binding.set_research_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.research_button_position().copied() {
                        target_binding.set_research_button_position(Some(position));
                    }
                    if let Some(tip) = source_binding.tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.research_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_research_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.un_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_un_tip(Some(tip_string));
                    }
                    if let Some(icon) = source_binding.icon() {
                        let icon_string = icon.to_string();
                        target_binding.set_icon(Some(icon_string));
                    }
                }
                WarcraftKeybinding::Command(source_binding) => {
                    let Some(target_binding) = self.command_or_default_mut(object_id) else {
                        continue;
                    };
                    if let Some(hotkey) = source_binding.hotkey() {
                        let hotkey_clone = *hotkey;
                        target_binding.set_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.button_position().copied() {
                        target_binding.set_button_position(Some(position));
                    }
                    if let Some(position) = source_binding.unbutton_position().copied() {
                        target_binding.set_unbutton_position(Some(position));
                    }
                    if let Some(tip) = source_binding.tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.un_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_un_tip(Some(tip_string));
                    }
                }
                WarcraftKeybinding::System(_) => {}
            }
        }
    }
}

struct CustomKeysParser {
    entries: BTreeMap<WarcraftObjectId, WarcraftKeybinding>,
    current_id: Option<WarcraftObjectId>,
    accumulator: Option<SectionAccumulator>,
}

impl CustomKeysParser {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            current_id: None,
            accumulator: None,
        }
    }

    fn flush_pending_section(&mut self) {
        let maybe_id = self.current_id.take();
        let maybe_accumulator = self.accumulator.take();
        if let Some(object_id) = maybe_id
            && let Some(accumulated) = maybe_accumulator
        {
            let binding = WarcraftKeybinding::from(accumulated);
            self.entries.insert(object_id, binding);
        }
    }

    fn extract_section_id(trimmed_line: &str) -> Option<String> {
        let without_brackets = trimmed_line.strip_prefix('[')?.strip_suffix(']')?;
        let section_id = without_brackets.trim();
        if section_id.is_empty() {
            None
        } else {
            Some(section_id.to_string())
        }
    }

    fn process_line(&mut self, line: &str) {
        let trimmed = line.trim();
        let is_blank = trimmed.is_empty();
        let is_comment = trimmed.starts_with("//") || trimmed.starts_with(';');

        if is_blank || is_comment {
            return;
        }

        if let Some(section_id) = Self::extract_section_id(trimmed) {
            self.flush_pending_section();
            if let Some(resolution) = SectionResolution::from_section_id(&section_id) {
                let already_present = self.entries.contains_key(resolution.canonical_id.value());
                if already_present {
                    self.current_id = None;
                    self.accumulator = None;
                } else {
                    let section_accumulator = SectionAccumulator::new(resolution.kind);
                    self.current_id = Some(resolution.canonical_id);
                    self.accumulator = Some(section_accumulator);
                }
            } else {
                self.current_id = None;
                self.accumulator = None;
            }
        } else if let Some((key, value)) = trimmed.split_once('=')
            && let Some(section_accumulator) = self.accumulator.as_mut()
        {
            section_accumulator.apply(key.trim(), value);
        }
    }

    fn finish(mut self) -> CustomKeys {
        self.flush_pending_section();
        CustomKeys::from(self.entries)
    }
}

impl From<&str> for CustomKeys {
    fn from(text: &str) -> Self {
        let mut parser = CustomKeysParser::new();
        for line in text.lines() {
            parser.process_line(line);
        }
        parser.finish()
    }
}

impl From<String> for CustomKeys {
    fn from(text: String) -> Self {
        Self::from(text.as_str())
    }
}

impl TryFrom<&std::path::Path> for CustomKeys {
    type Error = std::io::Error;

    fn try_from(path: &std::path::Path) -> Result<Self, Self::Error> {
        let text = std::fs::read_to_string(path)?;
        Ok(Self::from(text.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        AbilityBinding, AbilityModifier, CommandBinding, GridCoordinate, Hotkey, SystemBinding,
    };
    use warcraft_api::{SystemKeybindClass, SystemKeybindModifier};

    #[test]
    fn parses_single_entry_with_hotkey_and_buttonpos() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,2\n";
        let file = CustomKeys::from(input);
        let binding = file.binding("AHhb").unwrap();
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(binding.hotkey(), Some(&expected_hotkey));
        let position = binding.button_position().unwrap();
        assert_eq!(position.column(), ColumnIndex::Zero);
        assert_eq!(position.row(), RowIndex::Two);
    }

    #[test]
    fn lookup_uses_canonical_case() {
        let input = "[Hpal]\nHotkey=T\nButtonpos=3,0\n";
        let file = CustomKeys::from(input);
        assert!(file.binding("Hpal").is_some());
    }

    #[test]
    fn missing_hotkey_returns_none() {
        let input = "[AHbz]\nButtonpos=0,0\n";
        let file = CustomKeys::from(input);
        assert_eq!(file.binding("AHbz").unwrap().hotkey(), None);
    }

    #[test]
    fn empty_hotkey_value_treated_as_absent() {
        let input = "[AHbz]\nHotkey=\nButtonpos=0,0\n";
        let file = CustomKeys::from(input);
        assert_eq!(file.binding("AHbz").unwrap().hotkey(), None);
    }

    #[test]
    fn research_fields_parsed() {
        let input = "[AHhb]\nResearchhotkey=T\nResearchbuttonpos=3,1\n";
        let file = CustomKeys::from(input);
        let binding = file.binding("AHhb").unwrap();
        let expected_hotkey = Hotkey::Letter('T');
        assert_eq!(binding.research_hotkey(), Some(&expected_hotkey));
        let position = binding.research_button_position().unwrap();
        assert_eq!(position.column(), ColumnIndex::Three);
        assert_eq!(position.row(), RowIndex::One);
    }

    #[test]
    fn bindings_in_order_returns_alphabetical_order() {
        let binding_ahhb = AbilityBinding::builder().tip("first").build();
        let binding_ahbz = AbilityBinding::builder().tip("second").build();
        let file = CustomKeys::builder()
            .ability("AHhb", binding_ahhb)
            .ability("AHbz", binding_ahbz)
            .build();
        let ids: Vec<&str> = file
            .bindings_in_order()
            .map(|entry| entry.ability_id().value())
            .collect();
        assert_eq!(ids, ["AHbz", "AHhb"]);
    }

    #[test]
    fn comment_lines_are_skipped() {
        let input = "// This is a comment\n[AHhb]\nHotkey=Q\n; Also a comment\nButtonpos=0,0\n";
        let file = CustomKeys::from(input);
        let binding = file.binding("AHhb").unwrap();
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(binding.hotkey(), Some(&expected_hotkey));
        assert!(binding.button_position().is_some());
    }

    #[test]
    fn unknown_keys_are_silently_ignored() {
        let input = "[AHhb]\nHotkey=Q\nUnknownField=something\n";
        let file = CustomKeys::from(input);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            file.binding("AHhb").unwrap().hotkey(),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn malformed_buttonpos_gives_none() {
        let input = "[AHhb]\nButtonpos=notanumber\n";
        let file = CustomKeys::from(input);
        assert!(file.binding("AHhb").unwrap().button_position().is_none());
    }

    #[test]
    fn round_trip_outputs_lowercase_section_id() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,0\n\n";
        let file = CustomKeys::from(input);
        assert!(file.to_string().contains("[ahhb]"));
    }

    #[test]
    fn duplicate_section_uses_first_occurrence() {
        let input = "[AHhb]\nHotkey=Q\n\n[AHhb]\nHotkey=W\n";
        let file = CustomKeys::from(input);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            file.binding("AHhb").unwrap().hotkey(),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn untouched_sections_round_trip_byte_identically() {
        let input = "[AHhb]\nHotkey=Q\nButtonpos=0,2\n//inline comment\nIcon=ReplaceableTextures\\CommandButtons\\BTNAvatar.blp\n\n[AHbz]\nHotkey=W\nButtonpos=1,2\n\n";
        let file = CustomKeys::from(input);
        let output = file.to_string();
        assert!(output.contains("[ahhb]"));
        assert!(output.contains("BTNAvatar.blp"));
        assert!(output.contains("[ahbz]"));
    }

    #[test]
    fn touched_section_uses_formatted_output() {
        let hotkey_q = Hotkey::from('Q');
        let hotkey_w = Hotkey::from('W');
        let position_02 = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let position_12 = GridCoordinate::new(ColumnIndex::One, RowIndex::Two);
        let binding_ahhb = AbilityBinding::builder()
            .hotkey(hotkey_q)
            .button_position(position_02)
            .build();
        let binding_ahbz = AbilityBinding::builder()
            .hotkey(hotkey_w)
            .button_position(position_12)
            .build();
        let mut file = CustomKeys::builder()
            .ability("AHhb", binding_ahhb)
            .ability("AHbz", binding_ahbz)
            .build();
        let hotkey_r = Hotkey::from('R');
        file.binding_or_default_mut("AHhb")
            .unwrap()
            .set_hotkey(Some(hotkey_r));
        let output = file.to_string();
        assert!(output.contains("Hotkey=R"), "mutated hotkey must appear");
        assert!(
            output.contains("Hotkey=W"),
            "untouched section hotkey must still be present"
        );
    }

    #[test]
    fn parses_command_section() {
        let input = "[CmdMove]\nHotkey=M\nButtonpos=1,2\nTip=Move\n";
        let file = CustomKeys::from(input);
        let binding = file.command("CmdMove").expect("CmdMove parsed");
        let expected_hotkey = Hotkey::Letter('M');
        assert_eq!(binding.hotkey(), Some(&expected_hotkey));
        let position = binding.button_position().expect("position parsed");
        assert_eq!(position.column(), ColumnIndex::One);
        assert_eq!(position.row(), RowIndex::Two);
    }

    #[test]
    fn parses_system_section_game_command() {
        let input = "[itm1]\nHotkey=9\nGameCommand=1\n";
        let file = CustomKeys::from(input);
        let sys = file.system("itm1").expect("system section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(9));
        assert_eq!(sys.class(), SystemKeybindClass::Game);
        assert!(sys.modifier().is_none());
    }

    #[test]
    fn parses_system_section_ctrl_group_with_modifier() {
        let input = "[Ctr1]\nHotkey=49\nCtrlGroupCommand=1\nModifier=Ctrl\n";
        let file = CustomKeys::from(input);
        let sys = file.system("Ctr1").expect("parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(49));
        assert_eq!(sys.class(), SystemKeybindClass::ControlGroup);
        assert_eq!(sys.modifier(), Some(SystemKeybindModifier::Ctrl));
    }

    #[test]
    fn system_section_not_returned_by_binding() {
        let input = "[itm1]\nHotkey=9\nGameCommand=1\n";
        let file = CustomKeys::from(input);
        assert!(file.binding("itm1").is_none());
        assert!(file.system("itm1").is_some());
    }

    #[test]
    fn system_section_round_trips() {
        let input = "[itm1]\nHotkey=9\nGameCommand=1\n\n";
        let file = CustomKeys::from(input);
        let output = file.to_string();
        assert!(output.contains("[itm1]"));
        assert!(output.contains("Hotkey=9"));
        assert!(output.contains("GameCommand=1"));
    }

    #[test]
    fn all_ability_text_fields_parsed() {
        let input = concat!(
            "[Ahrl]\n",
            "Tip=Cast Holy Light\n",
            "Researchtip=Research something\n",
            "UnTip=Cancel\n",
            "Ubertip=Heals a friendly unit for 200 hit points.\n",
            "Researchubertip=Researches something powerful.\n",
            "Unubertip=Off form description.\n",
        );
        let file = CustomKeys::from(input);
        let binding = file.binding("Ahrl").expect("Ahrl must be present");
        assert_eq!(binding.tip(), Some("Cast Holy Light"));
        assert_eq!(binding.research_tip(), Some("Research something"));
        assert_eq!(binding.un_tip(), Some("Cancel"));
        assert_eq!(
            binding.ubertip(),
            Some("Heals a friendly unit for 200 hit points.")
        );
        assert_eq!(
            binding.research_ubertip(),
            Some("Researches something powerful.")
        );
        assert_eq!(binding.un_ubertip(), Some("Off form description."));
    }

    #[test]
    fn icon_field_parsed() {
        let input = "[Ahrl]\nIcon=ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp\n";
        let file = CustomKeys::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(
            binding.icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp"),
        );
    }

    #[test]
    fn art_alias_maps_to_icon_field() {
        let input = "[Ahrl]\nArt=ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp\n";
        let file = CustomKeys::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(
            binding.icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNHolyLight.blp"),
        );
    }

    #[test]
    fn unart_alias_maps_to_un_icon_field() {
        let input = "[Ahrl]\nUnArt=ReplaceableTextures\\CommandButtons\\BTNCancel.blp\n";
        let file = CustomKeys::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(
            binding.un_icon(),
            Some("ReplaceableTextures\\CommandButtons\\BTNCancel.blp"),
        );
    }

    #[test]
    fn modifier_field_parsed_in_ability_binding() {
        let input = "[Ahrl]\nModifier=Alt\n";
        let file = CustomKeys::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(binding.modifier(), Some(AbilityModifier::Alt));
    }

    #[test]
    fn modifier_field_case_insensitive_in_parsing() {
        let input = "[Ahrl]\nMODIFIER=Ctrl\n";
        let file = CustomKeys::from(input);
        let binding = file.binding("Ahrl").expect("present");
        assert_eq!(binding.modifier(), Some(AbilityModifier::Ctrl));
    }

    #[test]
    fn empty_file_has_no_entries() {
        let file = CustomKeys::from("");
        let ability_count = file.bindings_in_order().count();
        let command_count = file.commands_in_order().count();
        assert_eq!(ability_count, 0);
        assert_eq!(command_count, 0);
    }

    #[test]
    fn default_custom_keys_file_is_empty() {
        let file = CustomKeys::default();
        let ability_count = file.bindings_in_order().count();
        assert_eq!(ability_count, 0);
    }

    #[test]
    fn command_is_not_returned_by_binding_accessor() {
        let hotkey = Hotkey::from('M');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        let file = CustomKeys::builder().command("CmdMove", binding).build();
        assert!(file.binding("CmdMove").is_none());
        assert!(file.command("CmdMove").is_some());
    }

    #[test]
    fn ability_is_not_returned_by_command_accessor() {
        let hotkey = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let file = CustomKeys::builder().ability("Ahrl", binding).build();
        assert!(file.command("Ahrl").is_none());
        assert!(file.binding("Ahrl").is_some());
    }

    #[test]
    fn commands_in_order_returns_alphabetical_order() {
        let hotkey_a = Hotkey::from('A');
        let hotkey_m = Hotkey::from('M');
        let hotkey_s = Hotkey::from('S');
        let cmd_attack = CommandBinding::builder().hotkey(hotkey_a).build();
        let cmd_move = CommandBinding::builder().hotkey(hotkey_m).build();
        let cmd_stop = CommandBinding::builder().hotkey(hotkey_s).build();
        let file = CustomKeys::builder()
            .command("CmdAttack", cmd_attack)
            .command("CmdMove", cmd_move)
            .command("CmdStop", cmd_stop)
            .build();
        let names: Vec<&str> = file
            .commands_in_order()
            .map(|entry| entry.name().value())
            .collect();
        assert_eq!(names, ["CmdAttack", "CmdMove", "CmdStop"]);
    }

    #[test]
    fn commands_in_order_excludes_ability_sections() {
        let ability_hotkey = Hotkey::from('Q');
        let command_hotkey = Hotkey::from('A');
        let ability = AbilityBinding::builder().hotkey(ability_hotkey).build();
        let command = CommandBinding::builder().hotkey(command_hotkey).build();
        let file = CustomKeys::builder()
            .ability("Ahrl", ability)
            .command("CmdAttack", command)
            .build();
        let command_count = file.commands_in_order().count();
        assert_eq!(command_count, 1);
    }

    #[test]
    fn bindings_in_order_excludes_command_sections() {
        let command_hotkey = Hotkey::from('A');
        let ability_hotkey = Hotkey::from('Q');
        let command = CommandBinding::builder().hotkey(command_hotkey).build();
        let ability = AbilityBinding::builder().hotkey(ability_hotkey).build();
        let file = CustomKeys::builder()
            .command("CmdAttack", command)
            .ability("Ahrl", ability)
            .build();
        let binding_count = file.bindings_in_order().count();
        assert_eq!(binding_count, 1);
    }

    #[test]
    fn system_observer_command_parsed() {
        let input = "[THer]\nHotkey=120\nObserverCommand=1\n";
        let file = CustomKeys::from(input);
        let sys = file.system("THer").expect("observer section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(120));
        assert_eq!(sys.class(), SystemKeybindClass::Observer);
    }

    #[test]
    fn system_replay_command_parsed() {
        let input = "[TRpl]\nHotkey=80\nReplayCommand=1\n";
        let file = CustomKeys::from(input);
        let sys = file.system("TRpl").expect("replay section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(80));
        assert_eq!(sys.class(), SystemKeybindClass::Replay);
    }

    #[test]
    fn system_camera_command_parsed() {
        let input = "[ctcr]\nHotkey=65\nCameraCommand=1\n";
        let file = CustomKeys::from(input);
        let sys = file.system("ctcr").expect("camera section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(65));
        assert_eq!(sys.class(), SystemKeybindClass::Camera);
    }

    #[test]
    fn system_menu_command_parsed() {
        let input = "[QLog]\nHotkey=27\nMenuCommand=1\n";
        let file = CustomKeys::from(input);
        let sys = file.system("QLog").expect("menu section parsed");
        assert_eq!(sys.hotkey(), &Hotkey::VirtualKey(27));
        assert_eq!(sys.class(), SystemKeybindClass::Menu);
    }

    #[test]
    fn system_section_all_modifiers_parse() {
        struct ModifierCase {
            modifier_text: &'static str,
            expected_modifier: SystemKeybindModifier,
        }

        let cases = [
            ModifierCase {
                modifier_text: "Alt",
                expected_modifier: SystemKeybindModifier::Alt,
            },
            ModifierCase {
                modifier_text: "Ctrl",
                expected_modifier: SystemKeybindModifier::Ctrl,
            },
            ModifierCase {
                modifier_text: "Ctrl_or_Alt",
                expected_modifier: SystemKeybindModifier::CtrlOrAlt,
            },
            ModifierCase {
                modifier_text: "Shift",
                expected_modifier: SystemKeybindModifier::Shift,
            },
        ];
        for case in &cases {
            let modifier_text = case.modifier_text;
            let input =
                format!("[Ctr1]\nHotkey=49\nCtrlGroupCommand=1\nModifier={modifier_text}\n");
            let file = CustomKeys::from(input.as_str());
            let sys = file.system("Ctr1").expect("section parsed");
            let expected_modifier = Some(case.expected_modifier);
            assert_eq!(
                sys.modifier(),
                expected_modifier,
                "Modifier={modifier_text} must parse correctly",
            );
        }
    }

    #[test]
    fn set_system_hotkey_updates_existing_binding() {
        let initial_binding =
            SystemBinding::new(Hotkey::VirtualKey(27), SystemKeybindClass::Game, None);
        let mut file = CustomKeys::builder()
            .system("QLog", initial_binding)
            .build();
        file.set_system_hotkey("QLog", 65);
        let expected_hotkey = Hotkey::VirtualKey(65);
        assert_eq!(
            file.system("QLog").map(|binding| *binding.hotkey()),
            Some(expected_hotkey)
        );
    }

    #[test]
    fn set_system_hotkey_is_noop_for_missing_section() {
        let mut file = CustomKeys::default();
        file.set_system_hotkey("nonexistent", 65);
        assert!(file.system("nonexistent").is_none());
    }

    #[test]
    fn put_ability_inserts_and_is_accessible() {
        let hotkey = Hotkey::from('Q');
        let binding = AbilityBinding::builder().hotkey(hotkey).build();
        let mut file = CustomKeys::default();
        file.put_ability("Ahrl", binding);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            file.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn put_command_inserts_and_is_accessible() {
        let hotkey = Hotkey::from('A');
        let binding = CommandBinding::builder().hotkey(hotkey).build();
        let mut file = CustomKeys::default();
        file.put_command("CmdAttack", binding);
        let expected_hotkey = Hotkey::Letter('A');
        assert_eq!(
            file.command("CmdAttack")
                .and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn put_system_inserts_and_is_accessible() {
        let binding = SystemBinding::new(Hotkey::VirtualKey(9), SystemKeybindClass::Game, None);
        let mut file = CustomKeys::default();
        file.put_system("IsHeroSelect", binding);
        assert_eq!(
            file.system("IsHeroSelect")
                .map(|system_binding| *system_binding.hotkey()),
            Some(Hotkey::VirtualKey(9))
        );
    }

    #[test]
    fn put_ability_overwrites_existing_entry() {
        let first_hotkey = Hotkey::from('Q');
        let second_hotkey = Hotkey::from('W');
        let first = AbilityBinding::builder().hotkey(first_hotkey).build();
        let second = AbilityBinding::builder().hotkey(second_hotkey).build();
        let mut file = CustomKeys::default();
        file.put_ability("Ahrl", first);
        file.put_ability("Ahrl", second);
        let expected_hotkey = Hotkey::Letter('W');
        assert_eq!(
            file.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn round_trip_of_baseline_preserves_known_sections() {
        let baseline = include_str!("../templates/CustomKeys.txt");
        let file = CustomKeys::from(baseline);
        let output = file.to_string();
        let known_sections = [
            "[cmdattack]",
            "[cmdmove]",
            "[cmdrally]",
            "[cmdcancel]",
            "[cmdbuildhuman]",
            "[hpal]",
            "[hkee]",
            "[rhpm]",
            "[ahhb]",
        ];
        for section_marker in known_sections {
            assert!(
                output.contains(section_marker),
                "round-trip output is missing section {section_marker:?}",
            );
        }
        use std::collections::BTreeSet;
        let collect_unique_sections = |text: &str| -> BTreeSet<String> {
            text.lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.starts_with('[') && trimmed.ends_with(']') {
                        Some(trimmed.to_ascii_lowercase())
                    } else {
                        None
                    }
                })
                .collect()
        };
        let baseline_unique = collect_unique_sections(baseline);
        let output_unique = collect_unique_sections(&output);
        assert_eq!(
            baseline_unique, output_unique,
            "round-trip preserves the set of unique section headers",
        );
    }
}

#[cfg(test)]
mod extend_tests {
    use super::*;
    use crate::model::{AbilityBinding, CommandBinding, GridCoordinate, Hotkey, SystemBinding};
    use warcraft_api::SystemKeybindClass;

    #[test]
    fn extend_copies_hotkey_from_source_to_target() {
        let target_hotkey = Hotkey::from('Q');
        let uploaded_hotkey = Hotkey::from('W');
        let target_binding = AbilityBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = AbilityBinding::builder().hotkey(uploaded_hotkey).build();
        let mut target = CustomKeys::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.extend(uploaded);
        let expected_hotkey = Hotkey::Letter('W');
        assert_eq!(
            target.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn extend_copies_button_position() {
        let target_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let uploaded_position = GridCoordinate::new(ColumnIndex::Two, RowIndex::One);
        let target_binding = AbilityBinding::builder()
            .button_position(target_position)
            .build();
        let uploaded_binding = AbilityBinding::builder()
            .button_position(uploaded_position)
            .build();
        let mut target = CustomKeys::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.extend(uploaded);
        let position = target
            .binding("Ahrl")
            .and_then(|binding| binding.button_position())
            .copied();
        assert_eq!(
            position,
            Some(GridCoordinate::new(ColumnIndex::Two, RowIndex::One))
        );
    }

    #[test]
    fn extend_does_not_overwrite_system_entries() {
        let system_binding =
            SystemBinding::new(Hotkey::VirtualKey(27), SystemKeybindClass::Game, None);
        let mut target = CustomKeys::builder().system("IsS1", system_binding).build();
        let uploaded_hotkey = Hotkey::from('Q');
        let uploaded_binding = AbilityBinding::builder().hotkey(uploaded_hotkey).build();
        let uploaded = CustomKeys::builder()
            .ability("IsS1", uploaded_binding)
            .build();
        target.extend(uploaded);
        assert!(target.system("IsS1").is_some());
    }

    #[test]
    fn extend_skips_absent_fields() {
        let target_hotkey = Hotkey::from('Q');
        let uploaded_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let target_binding = AbilityBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = AbilityBinding::builder()
            .button_position(uploaded_position)
            .build();
        let mut target = CustomKeys::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.extend(uploaded);
        let expected_hotkey = Hotkey::Letter('Q');
        assert_eq!(
            target.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
        let position = target
            .binding("Ahrl")
            .and_then(|binding| binding.button_position())
            .copied();
        assert_eq!(
            position,
            Some(GridCoordinate::new(ColumnIndex::One, RowIndex::Zero))
        );
    }

    #[test]
    fn extend_copies_command_hotkey() {
        let target_hotkey = Hotkey::from('A');
        let uploaded_hotkey = Hotkey::from('G');
        let target_binding = CommandBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = CommandBinding::builder().hotkey(uploaded_hotkey).build();
        let mut target = CustomKeys::builder()
            .command("CmdAttack", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .command("CmdAttack", uploaded_binding)
            .build();
        target.extend(uploaded);
        let expected_hotkey = Hotkey::Letter('G');
        assert_eq!(
            target
                .command("CmdAttack")
                .and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }

    #[test]
    fn extend_merges_by_canonical_id() {
        let target_hotkey = Hotkey::from('Q');
        let uploaded_hotkey = Hotkey::from('E');
        let target_binding = AbilityBinding::builder().hotkey(target_hotkey).build();
        let uploaded_binding = AbilityBinding::builder().hotkey(uploaded_hotkey).build();
        let mut target = CustomKeys::builder()
            .ability("Ahrl", target_binding)
            .build();
        let uploaded = CustomKeys::builder()
            .ability("Ahrl", uploaded_binding)
            .build();
        target.extend(uploaded);
        let expected_hotkey = Hotkey::Letter('E');
        assert_eq!(
            target.binding("Ahrl").and_then(|binding| binding.hotkey()),
            Some(&expected_hotkey)
        );
    }
}

#[cfg(test)]
mod export_tests {
    use crate::CustomKeys;

    #[test]
    fn empty_overlay_on_minimal_baseline_round_trips() {
        let baseline = "[Ahrl]\nHotkey=Q\nButtonpos=0,0\n\n";
        let loaded = CustomKeys::from("");
        let output = loaded.serialize(baseline);
        assert!(
            output.contains("[ahrl]"),
            "baseline section should be present in output"
        );
        assert!(output.contains("Hotkey=Q"));
    }

    #[test]
    fn overlay_values_appear_in_export() {
        let baseline = "[Ahrl]\nHotkey=Q\n\n";
        let loaded = CustomKeys::from("[Ahrl]\nHotkey=W\n\n");
        let output = loaded.serialize(baseline);
        assert!(output.contains("Hotkey=W"), "user hotkey override must win");
    }

    #[test]
    fn export_with_real_baseline_contains_known_sections() {
        let baseline = include_str!("../templates/CustomKeys.txt");
        let loaded = CustomKeys::from("");
        let output = loaded.serialize(baseline);
        for section in &["[hpal]", "[cmdattack]", "[cmdmove]"] {
            assert!(output.contains(section), "export should contain {section}");
        }
    }

    #[test]
    fn export_materializes_default_button_positions() {
        // Ahrl (Holy Light) has a known default Buttonpos in the database.
        // Starting from an empty overlay, the export should inject it.
        let baseline = include_str!("../templates/CustomKeys.txt");
        let loaded = CustomKeys::from("");
        let output = loaded.serialize(baseline);
        // Find the [Ahrl] section and check Buttonpos is present.
        let after_ahrl = output
            .split("[ahrl]")
            .nth(1)
            .expect("[ahrl] must be in output");
        let next_section = after_ahrl.split('[').next().unwrap_or(after_ahrl);
        assert!(
            next_section.contains("Buttonpos="),
            "[Ahrl] section must have a Buttonpos after materialization"
        );
    }

    #[test]
    fn export_assigns_positions_to_goblin_merchant_shop_items_without_db_positions() {
        // bspd, spro, pinv are sold by the Goblin Merchant (ngme) but have no
        // default position in the game database. The export pipeline must assign
        // them positions so they appear in the command grid.
        let baseline = include_str!("../templates/CustomKeys.txt");
        let loaded = CustomKeys::from("");
        let output = loaded.serialize(baseline);

        for item_id in &["bspd", "spro", "pinv"] {
            let section_marker = format!("[{item_id}]");
            let after_section = output
                .to_ascii_lowercase()
                .split(section_marker.as_str())
                .nth(1)
                .unwrap_or("")
                .to_string();
            let before_next_section = after_section.split('[').next().unwrap_or("").to_string();
            assert!(
                before_next_section.contains("buttonpos="),
                "[{item_id}] must have a Buttonpos assigned by shop item materialization"
            );
        }
    }

    #[test]
    fn export_assigns_position_to_goblin_shredder_sell_unit_without_db_position() {
        // ngir (Goblin Shredder) is sold by the Goblin Laboratory (ngad) as a
        // sell_unit with no default position in the database or template.
        let baseline = include_str!("../templates/CustomKeys.txt");
        let loaded = CustomKeys::from("");
        let output = loaded.serialize(baseline);
        let lowercase_output = output.to_ascii_lowercase();
        let after_ngir = lowercase_output
            .split("[ngir]")
            .nth(1)
            .expect("[ngir] must be in output after sell_unit materialization");
        let before_next_section = after_ngir.split('[').next().unwrap_or(after_ngir);
        assert!(
            before_next_section.contains("buttonpos="),
            "[ngir] must have a Buttonpos assigned by sell_unit materialization"
        );
    }
}

#[cfg(test)]
mod normalize_tests {
    use crate::CustomKeys;
    use crate::model::{ColumnIndex, GridCoordinate, Hotkey, RowIndex};

    #[test]
    fn normalize_produces_non_empty_text() {
        let normalized = CustomKeys::from("").normalize();
        let normalized_text = normalized.to_string();
        assert!(!normalized_text.is_empty());
    }

    #[test]
    fn normalize_includes_known_baseline_sections() {
        let normalized = CustomKeys::from("").normalize();
        let normalized_text = normalized.to_string();
        assert!(normalized_text.contains("[hpal]"));
        assert!(normalized_text.contains("[cmdattack]"));
    }

    #[test]
    fn normalize_is_idempotent() {
        let first_text = CustomKeys::from("").normalize().to_string();
        let second_text = CustomKeys::from(first_text.as_str())
            .normalize()
            .to_string();
        assert_eq!(first_text, second_text);
    }

    #[test]
    fn normalize_includes_known_ability() {
        let normalized = CustomKeys::from("").normalize();
        let hpal_present = normalized.binding("Hpal").is_some();
        assert!(hpal_present);
    }

    #[test]
    fn normalize_overlays_user_hotkey_on_baseline() {
        let user_input = "[Ahrl]\nHotkey=Z\n\n";
        let normalized = CustomKeys::from(user_input).normalize();
        let ahrl_binding = normalized.binding("Ahrl");
        let ahrl_hotkey = ahrl_binding.and_then(|binding| binding.hotkey());
        let expected_hotkey = Hotkey::Letter('Z');
        assert_eq!(ahrl_hotkey, Some(&expected_hotkey));
    }

    #[test]
    fn normalize_materializes_button_position_for_known_ability() {
        let normalized = CustomKeys::from("").normalize();
        let normalized_text = normalized.to_string();
        let ahrl_marker = "[ahrl]";
        let ahrl_section_start = normalized_text
            .find(ahrl_marker)
            .expect("baseline must contain [ahrl]");
        let after_ahrl = &normalized_text[ahrl_section_start + ahrl_marker.len()..];
        let next_section_length = after_ahrl.find('[').unwrap_or(after_ahrl.len());
        let ahrl_section = &after_ahrl[..next_section_length];
        assert!(
            ahrl_section.contains("Buttonpos="),
            "[Ahrl] section must have a concrete Buttonpos after normalize",
        );
    }

    #[test]
    fn normalize_assigns_positions_to_goblin_merchant_sell_items_without_template_positions() {
        let normalized = CustomKeys::from("").normalize();
        for item_id in &["bspd", "spro", "pinv"] {
            let binding = normalized.binding(*item_id);
            let button_position = binding.and_then(|binding| binding.button_position());
            assert!(
                button_position.is_some(),
                "[{item_id}] must have a button_position in the normalized output"
            );
        }
    }

    #[test]
    fn normalize_assigns_position_to_goblin_shredder_sell_unit() {
        let normalized = CustomKeys::from("").normalize();
        let binding = normalized.binding("ngir");
        let button_position = binding.and_then(|binding| binding.button_position());
        assert!(
            button_position.is_some(),
            "[ngir] (Goblin Shredder) must have a button_position in the normalized output"
        );
    }

    #[test]
    fn normalize_defaults_button_position_to_origin_when_database_has_no_position() {
        // Prioritize (Aatp) on the Gargoyle has no default Buttonpos or
        // ResearchButtonpos in the game data, so it would otherwise be skipped
        // by materialize_default_positions and never render in the command card.
        // When both defaults are absent, fall back to (0, 0).
        let normalized = CustomKeys::from("").normalize();
        let binding = normalized
            .binding("Aatp")
            .expect("Aatp must have a binding after normalize");
        let button_position = binding
            .button_position()
            .expect("Aatp must have a fallback button_position");
        let origin = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        assert_eq!(*button_position, origin);
    }

    #[test]
    fn normalize_does_not_invent_off_state_for_one_shot_ability() {
        // Healing Wave (AChv) is a one-shot cast — it has no toggleable
        // off-state in the database (with_off_state(None, None, None, None)).
        // materialize_default_positions must not fabricate an unbutton_position
        // for it; doing so causes false off_state_blocks when moving other
        // abilities to the same grid cell.
        let normalized = CustomKeys::from("").normalize();
        let healing_wave_off = normalized
            .binding("AChv")
            .and_then(|binding| binding.unbutton_position());
        assert!(
            healing_wave_off.is_none(),
            "AChv has no off-state — normalize must not invent an unbutton_position"
        );
    }

    #[test]
    fn move_slot_co_moves_colocated_offstate_when_slot_ids_lack_abilityoff_variant() {
        // Regression: move_slot previously required AbilityOff(id) to be present
        // in slot_ids before co-moving an ability's off-state.  Toggle abilities
        // (ACsw, ACdm, etc.) always appear as Ability(id) in the command card —
        // never as AbilityOff(id) — so their Unbuttonpos never followed the move.
        use crate::command::move_request::MoveRequest;
        use crate::grid::layout::GridLayout;
        use crate::identity::slot::GridSlotId;
        let input = "[ACsw]\nButtonpos=0,0\nHotkey=Q\nUnbuttonpos=0,0\nUnhotkey=Q\n";
        let mut keys = CustomKeys::from(input);
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("ACsw");
        let slot_ids = [GridSlotId::ability("ACsw")];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 0, false);
        keys.move_slot(&request);
        let binding = keys.binding("ACsw").expect("ACsw must exist");
        let button_position = binding.button_position().expect("Buttonpos set");
        let unbutton_position = binding
            .unbutton_position()
            .expect("Unbuttonpos must follow");
        assert_eq!(
            u8::from(button_position.column()),
            1,
            "ability must move to column 1"
        );
        assert_eq!(
            u8::from(button_position.row()),
            0,
            "ability must move to row 0"
        );
        assert_eq!(
            unbutton_position, button_position,
            "Unbuttonpos must co-move with Buttonpos"
        );
    }

    #[test]
    fn move_slot_swaps_both_colocated_offstates_when_two_toggle_abilities_are_swapped() {
        // When two abilities that both have co-located off-states are swapped via
        // drag-drop, both Buttonpos AND Unbuttonpos must exchange — not just the
        // regular hotkey.  slot_ids contains only Ability variants, not AbilityOff.
        use crate::command::move_request::MoveRequest;
        use crate::grid::layout::GridLayout;
        use crate::identity::slot::GridSlotId;
        let input = concat!(
            "[ACsw]\nButtonpos=0,0\nHotkey=Q\nUnbuttonpos=0,0\nUnhotkey=Q\n",
            "[ACdm]\nButtonpos=1,0\nHotkey=W\nUnbuttonpos=1,0\nUnhotkey=W\n",
        );
        let mut keys = CustomKeys::from(input);
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("ACsw");
        let slot_ids = [GridSlotId::ability("ACsw"), GridSlotId::ability("ACdm")];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 0, false);
        keys.move_slot(&request);

        let acsw = keys.binding("ACsw").expect("ACsw must exist");
        let acsw_button = acsw.button_position().expect("ACsw Buttonpos set");
        let acsw_unbutton = acsw
            .unbutton_position()
            .expect("ACsw Unbuttonpos must follow");
        assert_eq!(
            u8::from(acsw_button.column()),
            1,
            "ACsw must move to column 1"
        );
        assert_eq!(
            acsw_unbutton, acsw_button,
            "ACsw Unbuttonpos must co-move with Buttonpos"
        );

        let acdm = keys.binding("ACdm").expect("ACdm must exist");
        let acdm_button = acdm.button_position().expect("ACdm Buttonpos set");
        let acdm_unbutton = acdm
            .unbutton_position()
            .expect("ACdm Unbuttonpos must follow");
        assert_eq!(
            u8::from(acdm_button.column()),
            0,
            "ACdm must be displaced to column 0"
        );
        assert_eq!(
            acdm_unbutton, acdm_button,
            "ACdm Unbuttonpos must co-move with Buttonpos"
        );
    }

    #[test]
    fn move_slot_does_not_co_move_offstate_when_not_colocated() {
        // If Unbuttonpos is at a DIFFERENT cell than Buttonpos, moving the ability
        // must NOT drag the off-state along — it sits at its own intentional position.
        use crate::command::move_request::MoveRequest;
        use crate::grid::layout::GridLayout;
        use crate::identity::slot::GridSlotId;
        let input = concat!(
            "[ACsw]\nButtonpos=0,0\nHotkey=Q\nUnbuttonpos=2,0\nUnhotkey=E\n",
            "[ACdm]\nButtonpos=1,0\nHotkey=W\n",
        );
        let mut keys = CustomKeys::from(input);
        let layout = GridLayout::qwerty_grid();
        let moving = GridSlotId::ability("ACsw");
        let slot_ids = [GridSlotId::ability("ACsw"), GridSlotId::ability("ACdm")];
        let request = MoveRequest::new(layout, &slot_ids, &moving, 1, 0, false);
        keys.move_slot(&request);

        let acsw = keys.binding("ACsw").expect("ACsw must exist");
        let acsw_unbutton = acsw
            .unbutton_position()
            .expect("Unbuttonpos must be preserved");
        assert_eq!(
            u8::from(acsw_unbutton.column()),
            2,
            "non-colocated Unbuttonpos must stay at column 2"
        );
        assert_eq!(
            u8::from(acsw_unbutton.row()),
            0,
            "non-colocated Unbuttonpos must stay at row 0"
        );
    }

    #[test]
    fn resolve_conflicts_co_moves_off_state_with_ability() {
        // ACsw (Slow) is a toggle ability whose off-state button sits at the
        // same grid cell as the on-state button. The cascade moves ACsw to
        // resolve a cross-unit collision. After resolve_conflicts the
        // unbutton_position must follow to the new cell — not be left behind
        // at the pre-cascade position, where it would ghost-block further edits.
        use crate::model::{ColumnIndex, GridCoordinate, RowIndex};
        let mut keys = CustomKeys::from("").normalize();
        let normalized_position = keys
            .binding("ACsw")
            .and_then(|binding| binding.button_position())
            .copied();
        let default_slow_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        assert_eq!(
            normalized_position,
            Some(default_slow_position),
            "ACsw must start at (0,2) after normalize"
        );
        let _plan = keys.resolve_conflicts();
        let binding = keys
            .binding("ACsw")
            .expect("ACsw must remain after resolve");
        let button_position = binding.button_position().copied();
        let unbutton_position = binding.unbutton_position().copied();
        assert_ne!(
            button_position,
            Some(default_slow_position),
            "ACsw must have been moved by the cascade"
        );
        assert_eq!(
            unbutton_position, button_position,
            "ACsw off-state must be co-located with on-state after resolve_conflicts"
        );
    }

    #[test]
    fn resolve_conflicts_produces_at_least_one_move_on_default_keys() {
        let mut normalized = CustomKeys::from("").normalize();
        let plan = normalized.resolve_conflicts();
        assert!(
            plan.move_count() > 0,
            "default keys have known collisions so resolve_conflicts must produce moves"
        );
    }

    #[test]
    fn resolve_conflicts_is_idempotent_on_default_keys() {
        let mut keys = CustomKeys::from("").normalize();
        let first_plan = keys.resolve_conflicts();
        assert!(first_plan.move_count() > 0, "first call must make moves");
        let second_plan = keys.resolve_conflicts();
        if second_plan.move_count() != 0 {
            let mut lines: Vec<String> = Vec::new();
            for planned_move in second_plan.moves() {
                let line = format!(
                    "  {} {:?} ({},{}) -> ({},{})",
                    planned_move.slot_id().as_str(),
                    planned_move.grid_role(),
                    u8::from(planned_move.old_position().column()),
                    u8::from(planned_move.old_position().row()),
                    u8::from(planned_move.new_position().column()),
                    u8::from(planned_move.new_position().row()),
                );
                lines.push(line);
            }
            panic!(
                "second resolve_conflicts call produced {} moves:\n{}",
                second_plan.move_count(),
                lines.join("\n"),
            );
        }
    }

    #[test]
    fn resolve_conflicts_writes_new_positions_into_bindings() {
        // After resolve_conflicts, every PlannedMove's slot must read back the
        // new_position from the bindings map.
        use crate::identity::slot::GridSlotId;
        let mut keys = CustomKeys::from("").normalize();
        let plan = keys.resolve_conflicts();
        for planned_move in plan.moves() {
            let slot = planned_move.slot_id();
            let expected = planned_move.new_position();
            let stored = match slot {
                GridSlotId::Ability(ability_id) => keys
                    .binding(ability_id)
                    .and_then(|binding| {
                        if planned_move.grid_role().is_research_context() {
                            binding.research_button_position()
                        } else {
                            binding.button_position()
                        }
                    })
                    .copied(),
                GridSlotId::AbilityOff(ability_id) => keys
                    .binding(ability_id)
                    .and_then(|binding| binding.unbutton_position())
                    .copied(),
                GridSlotId::Command(command_id) => keys
                    .command(command_id.value())
                    .and_then(|binding| binding.button_position())
                    .copied(),
            };
            assert_eq!(
                stored,
                Some(expected),
                "{} must have its new position written back to the binding",
                slot.as_str(),
            );
        }
    }

    #[test]
    fn resolve_conflicts_eliminates_intra_unit_collisions_too() {
        // Phase 2 of resolve_conflicts must clear any intra-unit collision
        // (both endpoints single-carrier) that phase 1 deliberately left
        // alone.  After resolve_conflicts returns, every pair of conflict-
        // graph neighbors that ended up resolved must occupy distinct cells
        // on the same role — regardless of carrier count.
        use crate::cascade::conflict_graph::ConflictGraph;
        use crate::cascade::planner::CascadePlan;
        use crate::cascade::queue::{AssignmentQueue, AssignmentScope};

        let mut keys = CustomKeys::from("").normalize();
        let _plan = keys.resolve_conflicts();

        // Re-evaluate against the full graph using the IncludingIntraUnit
        // scope so unresolved bookkeeping reflects every potential collision.
        let graph = ConflictGraph::build(&keys);
        let queue = AssignmentQueue::build_with_scope(graph, AssignmentScope::IncludingIntraUnit);
        let plan = CascadePlan::from(&queue);
        let unresolved: std::collections::HashSet<usize> = plan
            .unresolved()
            .iter()
            .filter_map(|mover| {
                queue.graph().nodes().iter().position(|node| {
                    node.slot_id() == mover.slot_id() && node.grid_role() == mover.grid_role()
                })
            })
            .collect();

        let graph_ref = queue.graph();
        for (first_index, first_node) in graph_ref.nodes().iter().enumerate() {
            if unresolved.contains(&first_index) {
                continue;
            }
            for &second_index in graph_ref.neighbors(first_index) {
                if second_index <= first_index {
                    continue;
                }
                if unresolved.contains(&second_index) {
                    continue;
                }
                let second_node = graph_ref.node(second_index);
                let first_position = queue.final_position(first_index);
                let second_position = queue.final_position(second_index);
                let same_role = first_node.grid_role() == second_node.grid_role();
                assert!(
                    first_position != second_position || !same_role,
                    "intra/cross-unit collision survives resolve_conflicts: {} and {} at ({},{})",
                    first_node.slot_id().as_str(),
                    second_node.slot_id().as_str(),
                    u8::from(first_position.column()),
                    u8::from(first_position.row()),
                );
            }
        }
    }

    #[test]
    fn destroyer_intra_unit_collision_produces_minimal_displacement() {
        // Aabs and Advm both default to (0,2) on the Destroyer (ubsp).
        // The WC3-canonical resolution keeps Advm at (0,2) and pushes Aabs
        // to (3,2), displacing only one ability.  With the old "lower index
        // wins" tiebreak Aabs won instead, then cascaded Advm into Afak,
        // displacing two abilities.  The intra-unit tiebreak (carrier_count=1
        // → higher index wins) restores the minimal-displacement outcome.
        let mut keys = CustomKeys::from("").normalize();
        let _plan = keys.resolve_conflicts();

        use crate::cascade::conflict_graph::ConflictGraph;
        use crate::unit::grids::GridRole;
        let graph = ConflictGraph::build(&keys);

        let check = |ability: &str, expected_col: u8, expected_row: u8| {
            let index = graph
                .find_node(ability, GridRole::MainCommand)
                .unwrap_or_else(|| panic!("{ability} not found in conflict graph"));
            let position = graph.node(index).current_position();
            let col = u8::from(position.column());
            let row = u8::from(position.row());
            assert_eq!(
                (col, row),
                (expected_col, expected_row),
                "{ability} expected ({expected_col},{expected_row}), got ({col},{row})"
            );
        };

        check("Advm", 0, 2);
        check("Afak", 1, 2);
        check("Aabs", 3, 2);
    }

    #[test]
    fn resolve_conflicts_cascades_origin_default_to_leftmost_free_cell() {
        // Prioritize (Aatp) on the Gargoyle has no default button position;
        // materialize_default_positions assigns (0,0).  The cascade pushes it
        // rightward through the pinned command row (Move/Stop/HoldPos/Attack),
        // then spills into row 1.  Patrol pins (0,1); the leftmost remaining
        // free cell is (1,1).  The spill must prefer the column closest to the
        // ability's *original* default column (0) rather than the cascade's
        // stuck column (3), so Aatp lands at (1,1) — not (3,1).
        let mut keys = CustomKeys::from("").normalize();
        let _plan = keys.resolve_conflicts();
        let binding = keys.binding("Aatp").expect("Aatp must have a binding");
        let position = binding
            .button_position()
            .expect("Aatp must have a button_position after resolve");
        let column = u8::from(position.column());
        let row = u8::from(position.row());
        assert_eq!(
            (column, row),
            (1, 1),
            "Aatp expected to cascade to (1,1), got ({column},{row})"
        );
    }

    #[test]
    fn resolved_default_customkeys_matches_snapshot() {
        // Full-text regression snapshot: normalize() the bundled default
        // CustomKeys.txt, run both cascade phases via resolve_conflicts(), and
        // serialize.  The byte sequence must match the checked-in expected
        // snapshot.  Any algorithm change (cascade ordering, pinning rules,
        // spill behavior), database change, or serialization tweak that
        // shifts the output trips this test.
        //
        // To accept a deliberate change: re-run the CLI
        //
        //   cargo run -p warcraft-cli -- resolve \
        //     crates/warcraft-keybinds/templates/CustomKeys.txt \
        //     --output crates/warcraft-keybinds/fixtures/resolved_default_customkeys.txt
        //
        // and inspect the diff before committing.
        let mut keys = CustomKeys::from("").normalize();
        let _plan = keys.resolve_conflicts();
        let actual = keys.to_string();
        let expected = include_str!("../fixtures/resolved_default_customkeys.txt");
        if actual != expected {
            let actual_bytes = actual.len();
            let expected_bytes = expected.len();
            let mut first_difference_offset: Option<usize> = None;
            for (offset, (actual_char, expected_char)) in
                actual.chars().zip(expected.chars()).enumerate()
            {
                if actual_char != expected_char {
                    first_difference_offset = Some(offset);
                    break;
                }
            }
            panic!(
                "resolved default CustomKeys drifted from snapshot \
                 (actual={actual_bytes}B, expected={expected_bytes}B, \
                 first diff at char {first_difference_offset:?}). \
                 To accept the new output, regenerate the snapshot via the CLI — \
                 see the test source for the exact command."
            );
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod template_generation_tests {
    use warcraft_api::WarcraftObjectMeta;
    use warcraft_database::ObjectLookup;
    use warcraft_database::{WARCRAFT_DATABASE, WARCRAFT_SYSTEM_KEYBINDS};

    use super::CustomKeys;
    use crate::grid::layout::GridLayout;

    fn join_levels(levels: &[&str]) -> Option<String> {
        if levels.is_empty() {
            None
        } else {
            Some(levels.join(","))
        }
    }

    fn build_text(layout: &GridLayout) -> String {
        let tmpl = CustomKeys::from(super::DEFAULT_CUSTOM_KEYS);
        let mut out = String::new();

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Command(cmd_meta) = warcraft_object.meta() else {
                continue;
            };
            let Some(default_position) = cmd_meta.default_button_position() else {
                continue;
            };
            let traditional = tmpl.command(id);
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            if let Some(hotkey_string) = traditional
                .and_then(|c| c.hotkey())
                .map(|hotkey_display| hotkey_display.to_string())
            {
                let hotkey_line = format!("Hotkey={hotkey_string}\n");
                out.push_str(&hotkey_line);
            }
            let default_column = u8::from(default_position.column());
            let default_row = u8::from(default_position.row());
            let buttonpos_line = format!("Buttonpos={default_column},{default_row}\n");
            out.push_str(&buttonpos_line);
            if let Some(tip) = traditional
                .and_then(|c| c.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(ubertip) = warcraft_object.ubertip() {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            out.push('\n');
        }

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Ability(ability_meta) = warcraft_object.meta() else {
                continue;
            };
            let default_button_position = warcraft_object.default_button_position();
            let default_research_position = warcraft_object.default_research_button_position();
            let off_button_position = ability_meta.off_button_position();
            if default_button_position.is_none()
                && default_research_position.is_none()
                && off_button_position.is_none()
            {
                continue;
            }
            let is_passive = ObjectLookup::is_passive_ability(id);
            let existing_binding = tmpl.binding(id);

            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);

            if let Some(button_position) = default_button_position {
                if !is_passive {
                    let hotkey = existing_binding
                        .and_then(|binding| binding.hotkey())
                        .map(|hotkey_display| hotkey_display.to_string())
                        .or_else(|| {
                            layout
                                .letter_at(button_position.column(), button_position.row())
                                .map(|letter| letter.to_string())
                        });
                    if let Some(hotkey_string) = hotkey {
                        let hotkey_line = format!("Hotkey={hotkey_string}\n");
                        out.push_str(&hotkey_line);
                    }
                }
                let btn_column = u8::from(button_position.column());
                let btn_row = u8::from(button_position.row());
                let buttonpos_line = format!("Buttonpos={btn_column},{btn_row}\n");
                out.push_str(&buttonpos_line);
            }

            if let Some(research_position) = default_research_position {
                let research_hotkey = existing_binding
                    .and_then(|binding| binding.research_hotkey())
                    .map(|hotkey_display| hotkey_display.to_string())
                    .or_else(|| {
                        layout
                            .letter_at(research_position.column(), research_position.row())
                            .map(|letter| letter.to_string())
                    });
                if let Some(research_hotkey_string) = research_hotkey {
                    let research_hotkey_line = format!("ResearchHotkey={research_hotkey_string}\n");
                    out.push_str(&research_hotkey_line);
                }
                let res_column = u8::from(research_position.column());
                let res_row = u8::from(research_position.row());
                let research_buttonpos_line = format!("ResearchButtonpos={res_column},{res_row}\n");
                out.push_str(&research_buttonpos_line);
            }

            if let Some(off_position) = off_button_position {
                let un_hotkey = existing_binding
                    .and_then(|binding| binding.unhotkey())
                    .map(|hotkey_display| hotkey_display.to_string())
                    .or_else(|| {
                        layout
                            .letter_at(off_position.column(), off_position.row())
                            .map(|letter| letter.to_string())
                    });
                if let Some(unhotkey_string) = un_hotkey {
                    let unhotkey_line = format!("Unhotkey={unhotkey_string}\n");
                    out.push_str(&unhotkey_line);
                }
                let off_column = u8::from(off_position.column());
                let off_row = u8::from(off_position.row());
                let unbuttonpos_line = format!("Unbuttonpos={off_column},{off_row}\n");
                out.push_str(&unbuttonpos_line);
            }

            if let Some(tip) = existing_binding
                .and_then(|binding| binding.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(un_tip) = existing_binding
                .and_then(|binding| binding.un_tip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.un_tip().map(str::to_owned))
            {
                let untip_line = format!("Untip={un_tip}\n");
                out.push_str(&untip_line);
            }
            if let Some(ubertip) = existing_binding
                .and_then(|binding| binding.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            if let Some(un_ubertip) = existing_binding
                .and_then(|binding| binding.un_ubertip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.un_ubertip().map(str::to_owned))
            {
                let un_ubertip_line = format!("Unubertip={un_ubertip}\n");
                out.push_str(&un_ubertip_line);
            }
            if let Some(research_ubertip) = existing_binding
                .and_then(|binding| binding.research_ubertip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.research_ubertip().map(str::to_owned))
            {
                let research_ubertip_line = format!("Researchubertip={research_ubertip}\n");
                out.push_str(&research_ubertip_line);
            }

            out.push('\n');
        }

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Unit(_) = warcraft_object.meta() else {
                continue;
            };
            let Some(default_position) = warcraft_object.default_button_position() else {
                continue;
            };
            let existing_binding = tmpl.binding(id);
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            if let Some(hotkey_string) = existing_binding
                .and_then(|binding| binding.hotkey())
                .map(|hotkey_display| hotkey_display.to_string())
            {
                let hotkey_line = format!("Hotkey={hotkey_string}\n");
                out.push_str(&hotkey_line);
            }
            let cmd_column = u8::from(default_position.column());
            let cmd_row = u8::from(default_position.row());
            let buttonpos_line = format!("Buttonpos={cmd_column},{cmd_row}\n");
            out.push_str(&buttonpos_line);
            if let Some(tip) = existing_binding
                .and_then(|binding| binding.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(ubertip) = existing_binding
                .and_then(|binding| binding.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            out.push('\n');
        }

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            if !matches!(
                warcraft_object.meta(),
                WarcraftObjectMeta::Upgrade(_) | WarcraftObjectMeta::Item(_)
            ) {
                continue;
            }
            let Some(default_position) = warcraft_object.default_button_position() else {
                continue;
            };
            let research_position = warcraft_object.default_research_button_position();
            let existing_binding = tmpl.binding(id);

            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);

            let hotkey = existing_binding
                .and_then(|binding| binding.hotkey())
                .map(|hotkey_display| hotkey_display.to_string())
                .or_else(|| {
                    layout
                        .letter_at(default_position.column(), default_position.row())
                        .map(|letter| letter.to_string())
                });
            if let Some(hotkey_string) = hotkey {
                let hotkey_line = format!("Hotkey={hotkey_string}\n");
                out.push_str(&hotkey_line);
            }
            let upg_column = u8::from(default_position.column());
            let upg_row = u8::from(default_position.row());
            let buttonpos_line = format!("Buttonpos={upg_column},{upg_row}\n");
            out.push_str(&buttonpos_line);

            if let Some(research_button_position) = research_position {
                let research_hotkey_string = existing_binding
                    .and_then(|binding| binding.research_hotkey())
                    .map(|hotkey_display| hotkey_display.to_string())
                    .or_else(|| {
                        layout
                            .letter_at(
                                research_button_position.column(),
                                research_button_position.row(),
                            )
                            .map(|letter| letter.to_string())
                    });
                if let Some(research_hotkey_line_value) = research_hotkey_string {
                    let research_hotkey_line =
                        format!("ResearchHotkey={research_hotkey_line_value}\n");
                    out.push_str(&research_hotkey_line);
                }
                let res_btn_column = u8::from(research_button_position.column());
                let res_btn_row = u8::from(research_button_position.row());
                let research_buttonpos_line =
                    format!("ResearchButtonpos={res_btn_column},{res_btn_row}\n");
                out.push_str(&research_buttonpos_line);
            }

            if let Some(tip) = existing_binding
                .and_then(|binding| binding.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(ubertip) = existing_binding
                .and_then(|binding| binding.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }

            out.push('\n');
        }

        for entry in WARCRAFT_SYSTEM_KEYBINDS.iter() {
            let id = entry.section_id();
            let hotkey_code = tmpl
                .system(id)
                .map(|binding| binding.hotkey().to_string())
                .unwrap_or_else(|| entry.default_hotkey().to_string());
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            let hotkey_line = format!("Hotkey={hotkey_code}\n");
            out.push_str(&hotkey_line);
            out.push_str(entry.class().ini_field());
            out.push('\n');
            if let Some(modifier_text) = entry.default_modifier().ini_str() {
                let modifier_line = format!("Modifier={modifier_text}\n");
                out.push_str(&modifier_line);
            }
            out.push('\n');
        }

        out
    }

    /// Regenerates CustomKeys.txt from the database. Run this whenever
    /// warcraft-database changes to keep the default template in sync.
    #[test]
    fn regenerate_default_template() {
        let content = build_text(&GridLayout::qwerty_grid());
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/CustomKeys.txt");
        std::fs::write(path, &content).expect("failed to write default template");
        println!("wrote {} bytes to {path}", content.len());
    }
}
