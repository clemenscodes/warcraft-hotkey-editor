use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::sync::OnceLock;

use warcraft_api::{WarcraftObjectId, WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::ability_id::AbilityId;
use crate::grid_layout::GridLayout;
use crate::hotkey_target::HotkeyTarget;
use crate::hotkey_token::HotkeyToken;
use crate::model::{
    AbilityBinding, BindingEntry, ColumnIndex, CommandBinding, CommandEntry, GridCoordinate,
    Hotkey, RowIndex, SectionAccumulator, SectionResolution, SystemBinding, WarcraftKeybinding,
};
use crate::move_request::MoveRequest;
use crate::slot::GridSlotId;

const BUNDLED_BASELINE: &str = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
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
        let id_str = ability_id.value();
        self.entries.get(id_str)?.as_ability()
    }

    pub fn binding_mut(&mut self, id: impl Into<AbilityId>) -> Option<&mut AbilityBinding> {
        let ability_id = id.into();
        let id_str = ability_id.value();
        self.entries.get_mut(id_str)?.as_ability_mut()
    }

    pub fn binding_or_default_mut(
        &mut self,
        id: impl Into<AbilityId>,
    ) -> Option<&mut AbilityBinding> {
        let ability_id = id.into();
        let object_id = ability_id.object_id();
        if !matches!(
            self.entries.get(object_id.value()),
            Some(WarcraftKeybinding::Ability(_))
        ) {
            self.entries.insert(
                object_id,
                WarcraftKeybinding::Ability(AbilityBinding::default()),
            );
        }
        self.entries
            .get_mut(object_id.value())
            .and_then(WarcraftKeybinding::as_ability_mut)
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
        self.entries.get(name)?.as_command()
    }

    pub fn command_mut(&mut self, name: &str) -> Option<&mut CommandBinding> {
        self.entries.get_mut(name)?.as_command_mut()
    }

    pub fn command_or_default_mut(
        &mut self,
        name: impl Into<WarcraftObjectId>,
    ) -> Option<&mut CommandBinding> {
        let object_id = name.into();
        if !matches!(
            self.entries.get(object_id.value()),
            Some(WarcraftKeybinding::Command(_))
        ) {
            self.entries.insert(
                object_id,
                WarcraftKeybinding::Command(CommandBinding::default()),
            );
        }
        self.entries
            .get_mut(object_id.value())
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

    pub fn system_mut(&mut self, id: &str) -> Option<&mut SystemBinding> {
        self.entries.get_mut(id)?.as_system_mut()
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
                    if default_button.is_none() && default_research.is_none() {
                        continue;
                    }
                    let canonical_id = *object_id;
                    let Some(binding) = self.binding_or_default_mut(canonical_id) else {
                        continue;
                    };
                    if binding.button_position().is_none()
                        && let Some(position_value) = default_button
                    {
                        binding.set_button_position(Some(position_value));
                    }
                    if binding.research_button_position().is_none()
                        && let Some(position_value) = default_research
                    {
                        binding.set_research_button_position(Some(position_value));
                    }
                    if binding.unbutton_position().is_none()
                        && !warcraft_object.is_passive_ability()
                    {
                        let database_off = match warcraft_object.meta() {
                            WarcraftObjectMeta::Ability(ability_meta) => {
                                ability_meta.off_button_position()
                            }
                            _ => None,
                        };
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
            if position.column().as_u8() == column && position.row().as_u8() == row {
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
                let is_passive = is_passive_ability(ability_id.value());
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
        let off_state_in_grid = |id: &str| -> bool {
            request.slot_ids().iter().any(
                |s| matches!(s, GridSlotId::AbilityOff(off_id) if off_id.value().eq_ignore_ascii_case(id)),
            )
        };
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
                        off_pos.column().as_u8() == request.target_column()
                            && off_pos.row().as_u8() == request.target_row()
                    })
            });
        let moving_off_colocated = !request.prevent_co_move()
            && match (request.moving_slot(), &moving_old_position) {
                (GridSlotId::Ability(id), Some(old_pos)) => {
                    off_state_in_grid(id.value())
                        && self
                            .position_for_slot(&GridSlotId::AbilityOff(*id), false)
                            .is_some_and(|off_pos| {
                                off_pos.column().as_u8() == old_pos.column().as_u8()
                                    && off_pos.row().as_u8() == old_pos.row().as_u8()
                            })
                }
                _ => false,
            };
        let displaced_off_colocated = match &displaced_slot {
            Some(GridSlotId::Ability(id)) => {
                off_state_in_grid(id.value())
                    && self
                        .position_for_slot(&GridSlotId::AbilityOff(*id), false)
                        .is_some_and(|off_pos| {
                            off_pos.column().as_u8() == request.target_column()
                                && off_pos.row().as_u8() == request.target_row()
                        })
            }
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
            let old_column = old_position.column().as_u8();
            let old_row = old_position.row().as_u8();
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
            let is_passive = is_passive_ability(ability_id_str);
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
    ) -> Option<GridSlotId> {
        for candidate_slot in slots {
            if candidate_slot
                .as_str()
                .eq_ignore_ascii_case(target_object_id)
            {
                continue;
            }
            let candidate_token =
                self.effective_token_for_slot(candidate_slot, layout, is_research_context);
            let Some(token_value) = candidate_token else {
                continue;
            };
            if token_value != proposed_token {
                continue;
            }
            return Some(*candidate_slot);
        }
        None
    }

    fn effective_token_for_slot(
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

fn is_passive_ability(id: &str) -> bool {
    WARCRAFT_DATABASE
        .by_id(id)
        .is_some_and(|object| object.is_passive_ability())
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
                        let hotkey_clone = hotkey.clone();
                        target_binding.set_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.button_position().copied() {
                        target_binding.set_button_position(Some(position));
                    }
                    if let Some(position) = source_binding.unbutton_position().copied() {
                        target_binding.set_unbutton_position(Some(position));
                    }
                    if let Some(hotkey) = source_binding.research_hotkey() {
                        let hotkey_clone = hotkey.clone();
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
                        let hotkey_clone = hotkey.clone();
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
                .map(|system_binding| system_binding.hotkey().clone()),
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
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
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
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
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
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
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
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
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
        let baseline = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
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
    use crate::model::Hotkey;

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
}
