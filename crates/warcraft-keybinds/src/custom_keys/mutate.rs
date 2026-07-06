//! The aggregate's mutation commands: the named write operations the renderer
//! invokes (assign/move positions, apply a grid layout, set/rewrite hotkeys).
//! Split out of the aggregate root as an `impl CustomKeys` continuation.

use super::{CustomKeys, HotkeyConflict};
use crate::command::move_request::MoveRequest;
use crate::grid::layout::GridLayout;
use crate::identity::ability_id::AbilityId;
use crate::identity::hotkey_target::HotkeyTarget;
use crate::identity::hotkey_token::HotkeyToken;
use crate::identity::slot::GridSlotId;
use crate::model::{ColumnIndex, GridCoordinate, Hotkey, RowIndex};
use warcraft_api::WarcraftObjectId;
use warcraft_database::{ObjectLookup, VariantUnits, WARCRAFT_DATABASE};

impl CustomKeys {
    pub fn assign_position(
        &mut self,
        layout: GridLayout,
        slot: &GridSlotId,
        column: u8,
        row: u8,
        is_research_context: bool,
        assign_hotkey: bool,
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
                let grid_hotkey = Self::grid_hotkey_for(*ability_id, letter);
                if let Some(binding) = self.binding_or_default_mut(*ability_id) {
                    if is_research_context {
                        binding.set_research_button_position(Some(new_position));
                        if assign_hotkey {
                            binding.set_research_hotkey(Some(grid_hotkey));
                        }
                    } else {
                        binding.set_button_position(Some(new_position));
                        if assign_hotkey && !is_passive {
                            binding.set_hotkey(Some(grid_hotkey));
                        }
                    }
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                if let Some(binding) = self.binding_or_default_mut(*ability_id) {
                    binding.set_unbutton_position(Some(new_position));
                    if assign_hotkey {
                        let unhotkey = Hotkey::from(letter);
                        binding.set_unhotkey(Some(unhotkey));
                    }
                }
            }
            GridSlotId::Command(command_name) => {
                if let Some(binding) = self.command_or_default_mut(*command_name) {
                    binding.set_button_position(Some(new_position));
                    if assign_hotkey {
                        let command_hotkey = Hotkey::from(letter);
                        binding.set_hotkey(Some(command_hotkey));
                    }
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
            request.assign_hotkey_on_move(),
        );
        if moving_off_colocated && let GridSlotId::Ability(moving_id) = request.moving_slot() {
            self.assign_position(
                request.layout(),
                &GridSlotId::AbilityOff(*moving_id),
                request.target_column(),
                request.target_row(),
                false,
                request.assign_hotkey_on_move(),
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
                request.assign_hotkey_on_move(),
            );
            if displaced_off_colocated && let GridSlotId::Ability(displaced_id) = &displaced {
                self.assign_position(
                    request.layout(),
                    &GridSlotId::AbilityOff(*displaced_id),
                    old_column,
                    old_row,
                    false,
                    request.assign_hotkey_on_move(),
                );
            }
        }
        if let GridSlotId::Ability(moving_id) = request.moving_slot() {
            let moving_ability_id = *moving_id;
            self.fan_out_position(moving_ability_id);
        }
        if let Some(GridSlotId::Ability(displaced_id)) = &displaced_slot {
            let displaced_ability_id = *displaced_id;
            self.fan_out_position(displaced_ability_id);
        }
    }

    fn fan_out_position(&mut self, ability_id: AbilityId) {
        let source_object_code = ability_id.value();
        let siblings = VariantUnits::fanout_siblings(source_object_code);
        if siblings.is_empty() {
            return;
        }
        let Some(source_binding) = self.binding(ability_id) else {
            return;
        };
        let button_position = source_binding.button_position().copied();
        let unbutton_position = source_binding.unbutton_position().copied();
        let research_button_position = source_binding.research_button_position().copied();
        for sibling_code in siblings.iter().copied() {
            let sibling_object_id = WarcraftObjectId::new(sibling_code);
            let sibling_ability_id = AbilityId::from(sibling_object_id);
            let Some(sibling_binding) = self.binding_or_default_mut(sibling_ability_id) else {
                continue;
            };
            sibling_binding.set_button_position(button_position);
            sibling_binding.set_unbutton_position(unbutton_position);
            sibling_binding.set_research_button_position(research_button_position);
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
                && binding
                    .hotkey()
                    .is_none_or(|hotkey| hotkey.accepts_grid_letter())
            {
                let new_hotkey = Self::grid_hotkey_for(bound_id, letter);
                if binding.hotkey() != Some(&new_hotkey) {
                    binding.set_hotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
            if let Some(position) = research_button_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && binding
                    .research_hotkey()
                    .is_none_or(|hotkey| hotkey.accepts_grid_letter())
            {
                let new_hotkey = Self::grid_hotkey_for(bound_id, letter);
                if binding.research_hotkey() != Some(&new_hotkey) {
                    binding.set_research_hotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
            if let Some(position) = unbutton_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && binding
                    .unhotkey()
                    .is_none_or(|hotkey| hotkey.accepts_grid_letter())
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
            if binding
                .hotkey()
                .is_none_or(|hotkey| hotkey.accepts_grid_letter())
            {
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
        self.apply_hotkey(target, new_token);
        let fan_out_ability_id = match target {
            HotkeyTarget::Ability(ability_id)
            | HotkeyTarget::AbilityResearch(ability_id)
            | HotkeyTarget::AbilityOffState(ability_id) => Some(ability_id),
            HotkeyTarget::Command(_) => None,
        };
        let Some(ability_id) = fan_out_ability_id else {
            return;
        };
        let source_object_code = ability_id.value();
        let siblings = VariantUnits::fanout_siblings(source_object_code);
        for sibling_code in siblings.iter().copied() {
            let sibling_object_id = WarcraftObjectId::new(sibling_code);
            let sibling_ability_id = AbilityId::from(sibling_object_id);
            let sibling_target = match target {
                HotkeyTarget::Ability(_) => HotkeyTarget::Ability(sibling_ability_id),
                HotkeyTarget::AbilityResearch(_) => {
                    HotkeyTarget::AbilityResearch(sibling_ability_id)
                }
                HotkeyTarget::AbilityOffState(_) => {
                    HotkeyTarget::AbilityOffState(sibling_ability_id)
                }
                HotkeyTarget::Command(_) => continue,
            };
            self.apply_hotkey(sibling_target, new_token);
        }
    }

    /// The number of hotkey tiers the live game expects for an upgrade, read
    /// from the game database. Multi-level upgrades (graveyard attack/armor,
    /// Necromancer/Banshee training) bind one comma-separated token per tier;
    /// replicating to fewer tiers leaves the follow-up levels without a working
    /// hotkey. Leveled abilities are excluded — they bind a single hotkey shared
    /// across levels (see [`WarcraftObject::upgrade_max_level`]). Non-upgrades
    /// yield `0`.
    fn upgrade_tier_count(ability_id: AbilityId) -> usize {
        let object_id = ability_id.object_id();
        let object_code = object_id.value();
        let object_option = WARCRAFT_DATABASE.by_id(object_code);
        object_option
            .and_then(|warcraft_object| warcraft_object.upgrade_max_level())
            .unwrap_or(0)
    }

    /// Builds the hotkey to assign when a grid letter lands on a binding. For a
    /// multi-level upgrade the letter is replicated to one token per tier
    /// (`Hotkey=F,F,F`); every other binding gets a single-letter hotkey.
    fn grid_hotkey_for(ability_id: AbilityId, letter: char) -> Hotkey {
        let token = HotkeyToken::try_from(letter).expect("grid layout letters are A to Z");
        let upgrade_levels = Self::upgrade_tier_count(ability_id);
        let tier_count = upgrade_levels.max(1);
        Hotkey::replicated(token, tier_count)
    }

    fn apply_hotkey(&mut self, target: HotkeyTarget, new_token: Option<HotkeyToken>) {
        match target {
            HotkeyTarget::Ability(ability_id) => {
                let upgrade_levels = Self::upgrade_tier_count(ability_id);
                if let Some(binding) = self.binding_or_default_mut(ability_id) {
                    let existing_levels = binding.hotkey().map_or(0, |hotkey| hotkey.level_count());
                    let tier_count = existing_levels.max(upgrade_levels).max(1);
                    let replicated = new_token.map(|token| Hotkey::replicated(token, tier_count));
                    binding.set_hotkey(replicated);
                }
            }
            HotkeyTarget::AbilityResearch(ability_id) => {
                let upgrade_levels = Self::upgrade_tier_count(ability_id);
                if let Some(binding) = self.binding_or_default_mut(ability_id) {
                    let research_levels = binding
                        .research_hotkey()
                        .map_or(0, |hotkey| hotkey.level_count());
                    let tier_count = research_levels.max(upgrade_levels).max(1);
                    let replicated = new_token.map(|token| Hotkey::replicated(token, tier_count));
                    binding.set_research_hotkey(replicated);
                }
            }
            HotkeyTarget::AbilityOffState(ability_id) => {
                if let Some(binding) = self.binding_or_default_mut(ability_id) {
                    let existing_levels =
                        binding.unhotkey().map_or(0, |hotkey| hotkey.level_count());
                    let replicated =
                        new_token.map(|token| Hotkey::replicated(token, existing_levels));
                    binding.set_unhotkey(replicated);
                }
            }
            HotkeyTarget::Command(command_name) => {
                if let Some(binding) = self.command_or_default_mut(command_name) {
                    let existing_levels = binding.hotkey().map_or(0, |hotkey| hotkey.level_count());
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
            let Some(token) = candidate_token else {
                continue;
            };
            if token != proposed_token {
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
        let resolved_position = self.position_for_slot(slot, is_research_context)?;
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
        let layout_letter =
            layout.letter_at(resolved_position.column(), resolved_position.row())?;
        HotkeyToken::try_from(layout_letter).ok()
    }
}
